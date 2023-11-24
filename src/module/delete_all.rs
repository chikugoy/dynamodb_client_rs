use std::time::Instant;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use crate::module::error::Error;

pub async fn delete_all_items(client: &Client) -> Result<(), Error> {
    let start = Instant::now();
    let table_name = "books";
    let mut exclusive_start_key = None;

    loop {
        let resp = client.scan()
            .table_name(table_name)
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await?;

        if let Some(items) = &resp.items {
            for item in items {
                let id_attr = match item.get("id") {
                    Some(attr) => attr,
                    None => return Err(Error::unhandled("item id not found".to_string())),
                };
                let id_value = match id_attr {
                    AttributeValue::S(s) => s,
                    _ => return Err(Error::unhandled("id attr error".to_string())),
                };

                let sort_attr = match item.get("sort") {
                    Some(attr) => attr,
                    None => return Err(Error::unhandled("item sort not found".to_string())),
                };
                let sort_value = match sort_attr {
                    AttributeValue::S(s) => s,
                    _ => return Err(Error::unhandled("sort attr error".to_string())),
                };

                client
                    .delete_item()
                    .table_name(table_name)
                    .key("id", AttributeValue::S(id_value.clone()))
                    .key("sort", AttributeValue::S(sort_value.clone()))
                    .send()
                    .await?;
            }
        }

        exclusive_start_key = resp.last_evaluated_key.clone();

        println!("Delete count");

        if exclusive_start_key.is_none() {
            break;
        }
    }

    println!("All items deleted successfully.");

    let duration = start.elapsed();
    let execution_time = duration.as_millis();
    println!("Execution time: {}ms", execution_time);

    Ok(())
}