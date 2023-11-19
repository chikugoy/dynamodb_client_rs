use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::operation::put_item::PutItemInput;
use aws_sdk_dynamodb::operation::get_item::GetItemInput;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::module::error::Error;

pub async fn put_item(client: &Client) -> Result<(), Error> {
    let put_item_input = PutItemInput::builder()
        .table_name("books")
        .item("id", AttributeValue::S("PrimaryKeyValue".to_string()))
        .item("sort", AttributeValue::S("SortKeyValue".to_string()))
        .build()
        .expect("Failed to build PutItemInput");

    client.put_item()
        .set_item(put_item_input.item().cloned())
        .table_name(put_item_input.table_name.unwrap_or_default())
        .send()
        .await?;

    println!("Item inserted successfully.");
    Ok(())
}

pub async fn get_item(client: &Client) -> Result<(), Error> {
    let get_item_input = GetItemInput::builder()
        .table_name("books")
        .key("id", AttributeValue::S("PrimaryKeyValue".to_string()))
        .key("sort", AttributeValue::S("SortKeyValue".to_string()))
        .build()
        .expect("Failed to build GetItemInput");

    let resp = client.get_item()
        .table_name(get_item_input.table_name.unwrap_or_default())
        .set_key(get_item_input.key)
        .send()
        .await?;

    if let Some(item) = resp.item {
        println!("Retrieved item: {:?}", item);
    } else {
        println!("Item not found.");
    }

    Ok(())
}
