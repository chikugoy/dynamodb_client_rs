use aws_sdk_dynamodb::{Client};
use rayon::prelude::*;
use std::time::Instant;
use crate::module::csv;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();

    let numbers: Vec<_> = (0..item_count).collect();

    let results: Vec<_> = numbers.par_chunks(25)
        .map(|chunk| {
            let client = client.clone();
            let mut requests = Vec::new();

            for i in chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                client.batch_write_item()
                    .request_items("books".to_string(), requests)
                    .send()
                    .await
            })
        })
        .collect();

    let aggregate_result = results.into_iter().fold(
        AggregateResult::new(),
        |mut acc, task_result| {
            match task_result {
                Ok(result) => acc.add_output_success(result),
                Err(e) => acc.add_sdk_error(e),
            };
            acc
        },
    );

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    aggregate_result.process_final_result();

    csv::write_to_csv("Parallel loop processing", item_count, execution_time).expect("parallel write_to_csv panic message");

    Ok(())
}
