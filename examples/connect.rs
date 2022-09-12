use std::time::Duration;

use airsim_client::{MultiRotorClient, NetworkResult};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.21.112.1:41451";
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // arm drone
    log::info!("arm drone");
    client.arm_disarm(true).await?;
    log::info!("Response: {:?}", res);

    // disarm drone
    log::info!("disarm drone");
    client.arm_disarm(false).await?;
    log::info!("Response: {:?}", res);

    // reset drone
    task::sleep(Duration::from_secs(1)).await;
    log::info!("reset drone");
    let res = client.reset().await?;
    log::info!("Response: {:?}", res);

    log::info!("Done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
