use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use aws_sdk_dynamodb::types::PutRequest;
use std::collections::HashMap;
use crate::module::error::Error;

pub async fn batch_write_items(client: &Client) -> Result<(), Error> {
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

    Ok(())
}
