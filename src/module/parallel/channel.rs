// Channels

use aws_sdk_dynamodb::{Client};
use tokio::sync::mpsc;
use std::time::Instant;
use tokio::task;
use crate::module::csv;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();

    let (sender, mut receiver) = mpsc::channel::<Result<_, Error>>(item_count / 25);
    let numbers = (0..item_count).collect::<Vec<_>>();

    for chunk in numbers.chunks(25).map(|c| c.to_vec()) {
        let sender = sender.clone();
        let client = client.clone();
        task::spawn(async move {
            let mut requests = Vec::new();
            for i in &chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            let result = client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await;

            let result = result.map_err(Error::from);
            sender.send(result).await.expect("sender send panic");
        });
    }

    drop(sender);

    let mut aggregate_result = AggregateResult::new();
    while let Some(result) = receiver.recv().await {
        match result {
            Ok(output) => aggregate_result.add_output_success(output),
            Err(e) => aggregate_result.add_error(e),
        }
    }

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    aggregate_result.process_final_result();

    csv::write_to_csv("Channel parallel processing", item_count, execution_time).expect("fork write_to_csv panic message");

    Ok(())
}
