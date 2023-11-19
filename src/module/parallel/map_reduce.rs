use aws_sdk_dynamodb::{Client};
use tokio::task;
use std::time::Instant;
use aws_sdk_dynamodb::operation::batch_write_item::{BatchWriteItemError, BatchWriteItemOutput};
use aws_smithy_runtime_api::http::Response;
use aws_smithy_runtime_api::client::result::SdkError;
use tokio::task::JoinError;
use crate::module::error::Error;
use crate::module::generate_request;

pub type HttpResponse = Response;

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
    let mut aggregate_result = AggregateResult::new(); // 結果集約用の構造体の初期化
    for task in tasks {
        match task.await {
            Ok(result) => {
                // 成功した書き込みの処理
                aggregate_result.add_success(result);
            }
            Err(e) => {
                // エラー処理
                aggregate_result.add_error(e);
            }
        }
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());

    // 最終的な集約結果の処理
    aggregate_result.process_final_result();

    Ok(())
}

// 結果集約用の構造体とそのメソッド
struct AggregateResult {
    // 必要なフィールドを定義
    // ...
}

impl AggregateResult {
    fn new() -> Self {
        AggregateResult {
            // ここに必要なフィールドの初期化を追加
        }
    }

    #[allow(unused_variables)]
    fn add_success(&mut self, result: Result<BatchWriteItemOutput, SdkError<
            BatchWriteItemError,
            HttpResponse,
        >>) {
        // 成功した書き込みの結果を集約
    }

    #[allow(unused_variables)]
    fn add_error(&mut self, error: JoinError) {
        // エラーを集約
    }

    fn process_final_result(&self) {
        // 最終的な集約結果の処理
    }
}
