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
    match args.get(1) {
        Some(process) if process == "scan" => module::scan_all_item::get_all_items(&client).await?,
        Some(process) if process == "count" => module::count_all_item::count_all_items(&client).await?,
        Some(process) if process == "batch" => module::batch::batch_write_items(&client).await?,
        Some(process) if process == "series_put" => module::series_process::put_item(&client).await?,
        Some(process) if process == "series_get" => module::series_process::get_item(&client).await?,
        Some(process) if process == "delete" => module::delete_all::delete_all_items(&client).await?,
        Some(process) if process == "fork_join" => module::parallel::fork_join::batch_write_items(&client).await?,
        _ => println!("Invalid argument. Please specify 'batch' or 'series'."),
    }

    Ok(())
}
