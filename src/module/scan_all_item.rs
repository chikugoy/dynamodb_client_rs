use aws_sdk_dynamodb::{Client};
use crate::module::error::Error;
use std::time::Instant;

pub async fn get_all_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let resp =
        client.scan().table_name("books").send().await?;

    if let Some(items) = resp.items {
        let duration = start.elapsed();
        let execution_time = duration.as_millis();
        println!("Execution time: {}ms", execution_time);

        for (i, item) in items.iter().enumerate() {
            println!("Retrieved item {}: {:?}", i + 1, item);
        }
    }

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    Ok(())
}