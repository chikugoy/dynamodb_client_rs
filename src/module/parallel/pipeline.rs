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

    // データ準備ステージ
    let numbers = (0..100).collect::<Vec<_>>();

    // チャンキングステージ
    let chunks = numbers.chunks(25).map(|c| c.to_vec()).collect::<Vec<_>>();

    // 非同期書き込みステージ
    let tasks = chunks.into_iter().map(|chunk| {
        let client = client.clone();
        task::spawn(async move {
            // リクエスト生成ステージ
            let requests = chunk.into_iter().map(|i| {
                generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string())
            }).collect::<Vec<_>>();

            // DynamoDBへのバッチ書き込み
            client.batch_write_item().request_items("books".to_string(), requests).send().await
        })
    }).collect::<Vec<_>>();

    // 結果集約ステージ
    let mut aggregate_result = AggregateResult::new();
    for task in tasks {
        match task.await {
            Ok(result) => aggregate_result.add_success(result),
            Err(e) => aggregate_result.add_error(e),
        }
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());
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