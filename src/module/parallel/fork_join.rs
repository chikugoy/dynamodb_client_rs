// Fork-Join

// use rand::Rng;
// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use std::sync::{Arc, Mutex};
// use tokio::time::{sleep, Duration};
use tokio::task;
use std::time::Instant;
use futures::future::join_all;
use aws_sdk_dynamodb::{Client};
use crate::module::csv;
use crate::module::error::Error;
use crate::module::generate_request;
use crate::module::aggregate_result::AggregateResult;

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();

    let numbers = (0..item_count).collect::<Vec<_>>();
    let mut tasks = Vec::new();

    // let rng = Arc::new(Mutex::new(StdRng::from_entropy()));

    for chunk in numbers.chunks(25).map(|c| c.to_vec()) {
        // let rng_clone = Arc::clone(&rng);
        let client = client.clone();
        let task = task::spawn(async move {
            let mut requests = Vec::new();
            for i in &chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            // let number = {
            //     let mut rng_guard = rng_clone.lock().unwrap();
            //     let number = rng_guard.gen_range(100..=2000);
            //     drop(rng_guard);
            //     number
            // };
            //
            // sleep(Duration::from_millis( number as u64)).await;

            client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await
        });

        tasks.push(task);
    }

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
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    aggregate_result.process_final_result();

    csv::write_to_csv("Fork join parallel processing", item_count, execution_time).expect("fork write_to_csv panic message");

    Ok(())
}
