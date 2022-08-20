use std::time::Duration;

use airsim_client::{MultiRotorClient, NetworkResult};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "127.0.0.1:41451";
    let client = MultiRotorClient::connect(address).await?;

    log::info!("Start!");

    log::info!("ping drone");
    let res = client.ping().await?;
    log::info!("Response: {:?}", res);

    task::sleep(Duration::from_secs(1)).await;
    log::info!("reset drone");
    let res = client.reset().await?;
    log::info!("Response: {:?}", res);

    task::sleep(Duration::from_secs(1)).await;
    log::info!("ping drone");
    let res = client.ping().await?;
    log::info!("Response: {:?}", res);

    log::info!("Done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
