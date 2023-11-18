// https://github.com/awslabs/aws-sdk-rust/blob/main/examples/examples/dynamodb/src/scenario/error.rs

/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use std::error::Error as StdError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unhandled error")]
    Unhandled(#[source] Box<dyn StdError + Send + Sync + 'static>),
}

impl Error {
    pub fn unhandled(source: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(source.into())
    }
}

impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(source: aws_sdk_dynamodb::Error) -> Self {
        Error::unhandled(source)
    }
}

impl<T> From<aws_sdk_dynamodb::error::SdkError<T>> for Error
    where
        T: StdError + Send + Sync + 'static,
{
    fn from(source: aws_sdk_dynamodb::error::SdkError<T>) -> Self {
        Error::unhandled(source)
    }
}