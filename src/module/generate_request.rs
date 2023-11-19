// dynamodb_utils.rs
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use aws_sdk_dynamodb::types::PutRequest;
use std::collections::HashMap;

pub fn create_write_request(id: String, sort_value: String) -> WriteRequest {
    let put_request = PutRequest::builder()
        .set_item(Some(HashMap::from([
            ("id".to_string(), AttributeValue::S(id)),
            ("sort".to_string(), AttributeValue::S(sort_value)),
        ])))
        .build()
        .expect("Failed to build PutRequest"); // エラーハンドリング

    WriteRequest::builder()
        .put_request(put_request)
        .build()
}
