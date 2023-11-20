use crate::module::csv;
use crate::module::batch;
use crate::module::{self, error::Error};
use aws_sdk_dynamodb::Client;

pub async fn parallel_performance(client: &Client, item_count: usize) -> Result<(), Error> {
    csv::write_to_csv_header("process", "item count", "execution time").expect("parallel_performance write_to_csv panic message");

    let count = 10;
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        batch::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::channel::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::fork_join::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::map_reduce::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::pipeline::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::r#loop::batch_write_items(client, item_count).await?;
    }
    for _ in 0..count {
        module::delete_all::delete_all_items(client).await?;
        module::parallel::worker::batch_write_items(client, item_count).await?;
    }

    Ok(())
}
