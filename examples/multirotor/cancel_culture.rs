use std::time::Duration;

use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Position3, YawMode};
use async_std::task;

use futures::future::FutureExt;

async fn cancel_ongoing_async() -> NetworkResult<()> {
    let address = "127.0.0.1:41451";
    let vehicle_name = "";

    // connect
    log::info!("connect for cancelling");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;
    task::sleep(Duration::from_secs(5)).await;

    let res = client.cancel_last_task().await?;
    log::info!("cancelled successfully: {res:?}");
    Ok(())
}
async fn connect_drone() -> NetworkResult<()> {
    let address = "127.0.0.1:41451";
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {res:?}");

    // arm drone
    log::info!("arm drone");
    let res = client.arm_disarm(true).await?;
    log::info!("Response: {res:?}");

    // Start the canceller at the background
    task::spawn(cancel_ongoing_async());

    // take off
    log::info!("take off drone");
    let _t1 = client.take_off_async(20.0).fuse().await?;
    let _t2 = task::sleep(Duration::from_secs(10)).fuse();

    log::info!("move to position");
    client
        .move_to_position_async(
            Position3::new(-10.0, 10.0, -100.0),
            3.0,
            1000.0,
            DrivetrainType::ForwardOnly,
            YawMode::new(false, 90.0),
            None,
            None,
        )
        .await?;

    // land drone
    log::info!("land drone");
    let res = client.land_async(20.0).await?;
    log::info!("Response: {res:?}");

    // disarm drone
    log::info!("disarm drone");
    let res = client.arm_disarm(false).await?;
    log::info!("Response: {res:?}");

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
