# DynamoDB Client for Rust

## Description

This Rust program provides a versatile interface to interact with AWS DynamoDB. It supports a variety of operations including scanning, counting, batch writing, and more, using the AWS SDK for DynamoDB. It's designed to demonstrate different ways to interact with DynamoDB, such as synchronous and asynchronous operations, various parallel processing methods, and performance measurement.

## Prerequisites

- Rust programming environment
- AWS account and credentials configured
- DynamoDB setup in the AWS environment
  - To add a table named books to DynamoDB with a partition key named id and a sort key named sort.
                                                                                                                                                                                                                                                                                                                        
## Installation

1. Clone the repository.
2. Navigate to the project directory.
3. Ensure AWS credentials are set up.
   - Rename .env.sample to .env and configure your ID and KEY.

## Usage

The program supports multiple operations with DynamoDB, which can be executed by passing arguments:

- `scan`: Scan all items from the DynamoDB table books.
- `count`: Count all items in the DynamoDB table books.
- `batch`: Perform batch write operations to DynamoDB table books.
- `series_put`: Put items into DynamoDB table books.
- `series_get`: Get items from DynamoDB table books.
- `delete`: Delete all items from the DynamoDB table books.
- parallel processing methods:
  - `channel`: Perform batch write operations using channel parallel processing.
  - `fork_join`: Use fork-join parallel processing for batch writes.
  - `map_reduce`: Implement map-reduce pattern for batch writes.
  - `pipeline`: Apply pipeline processing for batch writes.
  - `parallel_loop`: Execute batch writes in a parallel loop.
  - `worker`: Use worker/master pattern for batch writes.
  - `parallel_performance`: Measure the performance of these parallel processes.

Run the program with the desired operation and optional item count (default is 100 for parallel processing) as arguments:

```bash
cargo run [operation]
```

For example:

```bash
cargo run delete
cargo run channel 500
cargo run parallel_performance 1000
```

Please note that you will be charged for data input and output operations performed on DynamoDB.

## Performance

Data submission resulted in the following results. The average time (ms) for 10 trials of 1000 data submissions, respectively.

| Processing | Execution Time (ms) |
| ---- | ---- |
|Sequential processing | 619 |
|Channel parallel processing | 71 |
| Fork join parallel processing | 38|
| Map reduce parallel processing | 43　|
|Parallel loop processing | 146 |
| Worker/Master Pattern | 46|

## Contributing

Contributions to the project are welcome. Please follow the standard fork, branch, and pull request workflow.

## License

This project is licensed under the MIT License.

For more information on the MIT License, please visit https://opensource.org/licenses/MIT.