use aws_sdk_dynamodb::{Client};
use tokio::task;
use std::time::Instant;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let numbers = (0..100).collect::<Vec<_>>();
    let chunks = numbers.chunks(25).map(|c| c.to_vec()).collect::<Vec<_>>();

    let tasks = chunks.into_iter().map(|chunk| {
        let client = client.clone();
        task::spawn(async move {
            let requests = chunk.into_iter().map(|i| {
                generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string())
            }).collect::<Vec<_>>();

            client.batch_write_item().request_items("books".to_string(), requests).send().await
        })
    }).collect::<Vec<_>>();

    let mut aggregate_result = AggregateResult::new();
    for task in tasks {
        match task.await {
            Ok(result) => aggregate_result.add_success(result),
            Err(e) => aggregate_result.add_join_error(e),
        }
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}ms", duration.as_millis());
    aggregate_result.process_final_result();

    Ok(())
}
