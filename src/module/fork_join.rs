use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use aws_sdk_dynamodb::types::PutRequest;
use std::collections::HashMap;
use crate::module::error::Error;
use std::time::Instant;
use futures::future::join_all;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let numbers = (0..100).collect::<Vec<_>>();
    let mut tasks = Vec::new();

    for chunk in numbers.chunks(25) {
        let client = client.clone();
        let task = async move {
            let mut requests = Vec::new();
            for &i in chunk {
                let put_request = PutRequest::builder()
                    .set_item(Some(HashMap::from([
                        ("id".to_string(), AttributeValue::S(i.to_string())),
                        ("sort".to_string(), AttributeValue::S("SortKeyValue".to_string())),
                    ])))
                    .build()
                    .map_err(|e| Error::unhandled(e.to_string()))?; // エラーを適切に処理

                let write_request = WriteRequest::builder()
                    .put_request(put_request)
                    .build();

                requests.push(write_request);
            }

            client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await
                .map_err(|e| Error::unhandled(e.to_string()))?; // エラーを適切に処理

            Result::<(), Error>::Ok(())
        };
        tasks.push(task);
    }

    for result in join_all(tasks).await {
        result?; // エラー伝播
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());

    Ok(())
}
