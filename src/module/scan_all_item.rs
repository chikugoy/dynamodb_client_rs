use aws_sdk_dynamodb::{Client};
use crate::module::error::Error;

pub async fn get_all_items(client: &Client) -> Result<(), Error> {
    let resp =
        client.scan().table_name("books").send().await?;

    if let Some(item) = resp.items {
        println!("Retrieved item: {:?}", item);
    }

    Ok(())
}