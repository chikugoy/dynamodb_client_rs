// Worker/Master Pattern

use aws_sdk_dynamodb::{Client};
use tokio::{sync::mpsc, task};
use std::time::Instant;
use tokio::sync::mpsc::Receiver;
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
            Ok(res) => aggregate_result.add_output_success(res),
            Err(e) => aggregate_result.add_sdk_error(e),
        };
    }

    Ok(aggregate_result)
}

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();
    let (tx, rx) = mpsc::channel(32);

    for chunk in (0..item_count).collect::<Vec<_>>().chunks(25) {
        let chunk = chunk.to_vec();
        tx.send(chunk).await.unwrap();
    }

    drop(tx);

    let worker_handle = task::spawn(worker(client.clone(), rx));
    let aggregate_result = worker_handle.await??;

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    aggregate_result.process_final_result();

    csv::write_to_csv("Worker/Master pattern processing", item_count, execution_time).expect("worker/master write_to_csv panic message");

    Ok(())
}
