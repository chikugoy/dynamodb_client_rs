use aws_sdk_dynamodb::{Client};
use std::time::Instant;
use futures::future::join_all;
use crate::module::generate_request;
use crate::module::error::Error;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();

    let numbers = (0..100).collect::<Vec<_>>();
    let mut tasks = Vec::new();

    for chunk in numbers.chunks(25) {
        let client = client.clone();
        let task = async move {
            let mut requests = Vec::new();
            for &i in chunk {
                requests.push(generate_request::create_write_request(i.to_string(), "SortKeyValue".to_string()));
            }

            client
                .batch_write_item()
                .request_items("books".to_string(), requests)
                .send()
                .await
                .map_err(|e| Error::unhandled(e.to_string()))?;

            Result::<(), Error>::Ok(())
        };
        tasks.push(task);
    }

    for result in join_all(tasks).await {
        result?;
    }

    let duration = start.elapsed();
    println!("実行時間: {:?}ms", duration.as_millis());

    Ok(())
}
