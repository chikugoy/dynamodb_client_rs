use std::time::Instant;
use std::collections::HashMap;
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::WriteRequest;
use aws_sdk_dynamodb::operation::batch_write_item::{BatchWriteItemError, BatchWriteItemOutput};
use aws_smithy_runtime_api::http::Response;
use aws_smithy_runtime_api::client::result::SdkError;
use tokio::task::JoinError;
use crate::module::error::Error;

pub type HttpResponse = Response;

const TOTAL_REQUEST_COUNT: usize = 25;

pub(crate) struct AggregateResult {
    success_count: usize,
    unprocessed_count: usize,
    error_messages: Vec<String>,
    unprocessed_items: HashMap<String, Vec<WriteRequest>>,
    retry_success_count: usize,
    retry_error_messages: Vec<String>,
}

impl AggregateResult {
    pub(crate) fn new() -> Self {
        AggregateResult {
            success_count: 0,
            unprocessed_count: 0,
            error_messages: Vec::new(),
            unprocessed_items: HashMap::new(),
            retry_success_count: 0,
            retry_error_messages: Vec::new(),
        }
    }

    pub(crate) fn add_success(&mut self, result: Result<BatchWriteItemOutput, SdkError<BatchWriteItemError, HttpResponse>>) {
        if let Ok(output) = result {
            let unprocessed_items = output.unprocessed_items.unwrap_or_default();

            for (table_name, requests) in &unprocessed_items {
                self.unprocessed_items.entry(table_name.clone()).or_insert_with(Vec::new).extend(requests.clone());
            }

            let unprocessed_count: usize = unprocessed_items.values().map(|requests| requests.len()).sum();

            self.unprocessed_count += unprocessed_count;

            let processed_count = TOTAL_REQUEST_COUNT - unprocessed_count;
            self.success_count += processed_count;
        }
    }

    pub(crate) fn add_output_success(&mut self, output: BatchWriteItemOutput) {
        let unprocessed_count = output.unprocessed_items.map_or(0, |unprocessed| {
            unprocessed.values().map(|requests| requests.len()).sum()
        });

        let processed_count = TOTAL_REQUEST_COUNT - unprocessed_count;
        self.success_count += processed_count;
    }

    pub(crate) fn add_join_error(&mut self, error: JoinError) {
        let error_message = error.to_string();
        self.error_messages.push(error_message);
    }

    pub(crate) fn add_error(&mut self, error: Error) {
        let error_message = error.to_string();
        self.error_messages.push(error_message);
    }

    pub(crate) fn add_sdk_error(&mut self, error: SdkError<BatchWriteItemError, Response>) {
        let error_message = error.to_string();
        self.error_messages.push(error_message);
    }

    pub async fn retry_unprocessed_items(&mut self, client: &Client, table_name: &str) {
        if self.unprocessed_items.is_empty() {
            return;
        }

        let start = Instant::now();

        for requests in self.unprocessed_items.values() {
            for unprocessed_item in requests {
                let retry_result = client.batch_write_item()
                    .request_items(table_name.to_string(), vec![unprocessed_item.clone()])
                    .send()
                    .await;

                match retry_result {
                    Ok(_) => {
                        self.retry_success_count += 1;
                    }
                    Err(e) => {
                        let error_message = e.to_string();
                        self.retry_error_messages.push(error_message);
                    }
                }
            }
        }

        self.unprocessed_items.clear();

        let duration = start.elapsed();
        let execution_time = duration.as_millis();
        println!("Retry unprocessed execution time: {}ms", execution_time);
    }

    pub(crate) fn process_final_result(&self) {
        println!("Number of successful items: {}", self.success_count + self.retry_success_count);

        if self.unprocessed_count > 0 {
            println!("Number of unprocessed items: {}", self.unprocessed_count);
        }

        if self.error_messages.is_empty() {
            return;
        }

        println!("Error messages:");
        for message in &self.error_messages {
            println!(" - {}", message);
        }
    }
}
