use aws_sdk_dynamodb::{Client};
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_config;
use dotenv::dotenv;
use std::env;

mod module;

#[tokio::main]
async fn main() -> Result<(), module::error::Error> {
    dotenv().ok();

    let region_provider = RegionProviderChain::default_provider();
    let shared_config = aws_config::defaults(BehaviorVersion::v2023_11_09()).region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let args: Vec<String> = env::args().collect();
    let process = args.get(1).map(String::as_str);
    let item_count: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);
    match process {
        Some("scan") => module::scan_all_item::get_all_items(&client).await?,
        Some("count") => module::count_all_item::count_all_items(&client).await?,
        Some("batch") => module::batch::batch_write_items(&client, item_count).await?,
        Some("series_put") => module::series_process::put_item(&client).await?,
        Some("series_get") => module::series_process::get_item(&client).await?,
        Some("delete") => module::delete_all::delete_all_items(&client).await?,
        Some("channel") => module::parallel::channel::batch_write_items(&client, item_count).await?,
        Some("fork_join") => module::parallel::fork_join::batch_write_items(&client, item_count).await?,
        Some("map_reduce") => module::parallel::map_reduce::batch_write_items(&client, item_count).await?,
        Some("pipeline") => module::parallel::pipeline::batch_write_items(&client, item_count).await?,
        Some("parallel_performance") => module::parallel::performance::parallel_performance(&client, item_count).await?,
        _ => println!("Invalid argument. Please specify 'batch' or 'series'."),
    }

    Ok(())
}
