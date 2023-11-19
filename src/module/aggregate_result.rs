use aws_sdk_dynamodb::operation::batch_write_item::{BatchWriteItemError, BatchWriteItemOutput};
use aws_smithy_runtime_api::http::Response;
use aws_smithy_runtime_api::client::result::SdkError;
use tokio::task::JoinError;
use crate::module::error::Error;

pub type HttpResponse = Response;

const TOTAL_REQUEST_COUNT: usize = 25;

pub(crate) struct AggregateResult {
    success_count: usize,
    error_messages: Vec<String>,
}

impl AggregateResult {
    pub(crate) fn new() -> Self {
        AggregateResult {
            success_count: 0,
            error_messages: Vec::new(),
        }
    }

    pub(crate) fn add_success(&mut self, result: Result<BatchWriteItemOutput, SdkError<BatchWriteItemError, HttpResponse>>) {
        match result {
            Ok(output) => {
                let unprocessed_count = output.unprocessed_items.map_or(0, |unprocessed| {
                    unprocessed.values().map(|requests| requests.len()).sum()
                });

                let processed_count = TOTAL_REQUEST_COUNT - unprocessed_count;
                self.success_count += processed_count;
            }
            Err(_) => {}
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

    pub(crate) fn process_final_result(&self) {
        println!("Number of successful items: {}", self.success_count);

        if self.error_messages.is_empty() {
            return;
        }

        println!("Error messages:");
        for message in &self.error_messages {
            println!(" - {}", message);
        }
    }
}
