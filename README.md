# dynamodb_client_rs

https://github.com/awslabs/aws-sdk-rust/tree/main/examples/examples/dynamodb
https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/
https://dynobase.dev/dynamodb-rust/#batch-put-item

### series processing
cargo run batch
100 items
Execution time: 227ms

### fork join parallel processing
cargo run fork_join
100 items
Execution time: 77ms

### channel parallel processing
cargo run channel
100 items
Execution time: 77ms

25件の1リクエストだけを並列にしているので、件数を増やしてもよさそう