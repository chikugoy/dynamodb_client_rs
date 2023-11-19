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
                .unwrap();

            sender.send(result).await.unwrap(); // 結果を送信
        });
    }

    for _ in 0..4 {
        receiver.recv().await.unwrap();
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());

    Ok(())
}
