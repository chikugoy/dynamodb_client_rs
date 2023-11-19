use aws_sdk_dynamodb::{Client};
use tokio::task;
use std::time::Instant;
use futures::future::join_all;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let numbers = (0..100).collect::<Vec<_>>();
    let mut tasks = Vec::new();

    // マップフェーズ: 各チャンクに対して非同期タスクを生成
    for chunk in numbers.chunks(25).map(|c| c.to_vec()) {
        let client = client.clone();
        let task = task::spawn(async move {
            let mut requests = Vec::new();
            for i in &chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            // DynamoDBへのバッチ書き込み
            client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await
        });

        tasks.push(task);
    }

    // リデュースフェーズ: 各タスクの結果を集約
    let aggregate_result = join_all(tasks).await.into_iter().fold(
        AggregateResult::new(),
        |mut acc, task_result| {
            match task_result {
                Ok(result) => acc.add_success(result),
                Err(e) => acc.add_join_error(e),
            };
            acc
        },
    );

    let duration = start.elapsed();
    println!("Execution time: {:?}ms", duration.as_millis());

    // 最終的な集約結果の処理
    aggregate_result.process_final_result();

    Ok(())
}
