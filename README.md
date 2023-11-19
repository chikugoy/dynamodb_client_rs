# dynamodb_client_rs

https://github.com/awslabs/aws-sdk-rust/tree/main/examples/examples/dynamodb
https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/
https://dynobase.dev/dynamodb-rust/#batch-put-item

### 直列
cargo run batch
100件
実行時間: 227ms

### フォークジョイン並列
cargo run fork_join
100件
実行時間: 77ms

### チャンネル
cargo run channel
100件
実行時間: 77ms
