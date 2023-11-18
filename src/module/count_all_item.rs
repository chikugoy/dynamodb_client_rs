use aws_sdk_dynamodb::{Client};
use crate::module::error::Error;

pub async fn count_all_items(client: &Client) -> Result<(), Error> {
    let resp = client.scan().table_name("books").send().await?;

    if let Some(items) = resp.items {
        println!("Retrieved items count: {}", items.len());
    } else {
        println!("No items found.");
    }

    Ok(())
}