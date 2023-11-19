use aws_sdk_dynamodb::{Client};
use tokio::sync::mpsc;
use std::time::Instant;
use tokio::task;
use crate::module::error::Error;
use crate::module::generate_request;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let (sender, mut receiver) = mpsc::channel(4); // チャンネルのサイズを設定
    let numbers = (0..100).collect::<Vec<_>>();

    for chunk in numbers.chunks(25).map(|c| c.to_vec()) {
        let sender = sender.clone();
        let client = client.clone();
        task::spawn(async move {
            let mut requests = Vec::new();
            for i in &chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            // DynamoDBへのバッチ書き込み
            let result = client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await
                .unwrap(); // エラーハンドリングが必要

            sender.send(result).await.unwrap(); // 結果を送信
        });
    }

    // 結果の集約
    for _ in 0..4 { // 4回のチャンクに対応
        let result = receiver.recv().await.unwrap(); // 結果の受信
        // 結果の処理...
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());

    Ok(())
}
