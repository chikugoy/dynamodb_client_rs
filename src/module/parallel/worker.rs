// Worker/Master Pattern

use aws_sdk_dynamodb::{Client};
use tokio::{sync::mpsc, task};
use std::time::Instant;
use tokio::sync::mpsc::Receiver;
use futures::future::join_all;
use crate::module::csv;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

async fn worker(client: Client, mut receiver: Receiver<Vec<usize>>) -> Result<AggregateResult, Error> {
    let mut aggregate_result = AggregateResult::new();

    while let Some(chunk) = receiver.recv().await {
        let requests = chunk.into_iter()
            .map(|i| generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()))
            .collect::<Vec<_>>();

        let result = client.batch_write_item()
            .request_items("books".to_string(), requests)
            .send()
            .await;

        match result {
            Ok(res) => {
                aggregate_result.add_output_success(res)
            },
            Err(e) => {
                aggregate_result.add_sdk_error(e)
            },
        };
    }

    Ok(aggregate_result)
}

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();

    let worker_count = item_count / 25;
    let chunk_size = item_count / worker_count;
    let mut handles = Vec::new();

    for i in 0..worker_count {
        let (tx, rx) = mpsc::channel(32);
        let worker_client = client.clone();

        let start_index = i * chunk_size;
        let end_index = start_index + chunk_size;
        for chunk in (start_index..end_index).collect::<Vec<_>>().chunks(25) {
            let chunk = chunk.to_vec();
            tx.send(chunk).await.unwrap();
        }

        let handle = task::spawn(worker(worker_client, rx));
        handles.push(handle);
    }

    let results = join_all(handles).await;
    for result in results {
        match result {
            Ok(_aggregate_result) => {
                // Success process
            }
            Err(_e) => {
                // Error process
            }
        }
    }

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    csv::write_to_csv("Worker/Master pattern processing", item_count, execution_time).expect("worker/master write_to_csv panic message");

    Ok(())
}
