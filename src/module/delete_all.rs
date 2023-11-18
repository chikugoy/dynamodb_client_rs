use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::AttributeValue;
use crate::module::error::Error;

pub async fn delete_all_items(client: &Client) -> Result<(), Error> {
    let table = "books";
    let resp = client.scan().table_name(table).send().await?;

    if let Some(items) = resp.items {
        for item in items {
            let id_attr = match item.get("id") {
                Some(attr) => attr,
                None => {
                    return Err(Error::unhandled("item id not found".to_string()));
                }
            };
            let id_value = match id_attr {
                AttributeValue::S(s) => s,
                _ => return Err(Error::unhandled("id attr error".to_string())),
            };
            println!("Retrieved item id: {:?}", id_value);

            let sort_attr = match item.get("sort") {
                Some(attr) => attr,
                None => {
                    return Err(Error::unhandled("item sort not found".to_string()));
                }
            };
            let sort_value = match sort_attr {
                AttributeValue::S(s) => s,
                _ => return Err(Error::unhandled("sort attr error".to_string())),
            };
            println!("Retrieved item sort: {:?}", sort_value);

            client
                .delete_item()
                .table_name(table)
                .key("id", AttributeValue::S(id_value.clone()))
                .key("sort", AttributeValue::S(sort_value.clone()))
                .send()
                .await?;
            println!("Deleted item from table");
        }
    }

    println!("All items deleted successfully.");
    Ok(())
}