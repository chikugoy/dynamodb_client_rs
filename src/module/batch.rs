use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use aws_sdk_dynamodb::types::PutRequest;
use std::collections::HashMap;
use crate::module::error::Error;
use std::time::Instant;
use crate::module::csv;

pub async fn batch_write_items(client: &Client, item_count: usize) -> Result<(), Error> {
    let start = Instant::now();

    let mut requests = Vec::new();
    for i in 0..100 {
        let put_request_result = PutRequest::builder()
            .set_item(Some(HashMap::from(
                [
                    (
                        "id".to_string(),
                        AttributeValue::S(i.to_string()),
                    ),
                    (
                        "sort".to_string(),
                        AttributeValue::S("SortKeyValue".to_string()),
                    ),
                ],
            )))
            .build();

        if let Err(error) = put_request_result {
            eprintln!("Error creating PutRequest: {:?}", error);
            std::process::exit(1);
        }

        let put_request = put_request_result.unwrap();
        let write_request = WriteRequest::builder()
            .put_request(put_request)
            .build();

        requests.push(write_request);

        if requests.len() == 25 {
            client
                .batch_write_item()
                .request_items("books".to_string(), requests.clone())
                .send()
                .await?;

            requests.clear();
        }
    }

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    csv::write_to_csv("Batch sequential processing", item_count, execution_time).expect("fork write_to_csv panic message");

    Ok(())
}
