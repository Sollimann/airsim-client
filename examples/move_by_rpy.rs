use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Orientation3, RCData, Velocity3, YawMode};
use std::{sync::Arc, thread, time::Duration};
// use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.21.176.1:41451"; // set with env variable
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let _client = MultiRotorClient::connect(address, vehicle_name).await?;
    let client = Arc::new(_client);
    let client_clone = client.clone();

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // arm drone
    log::info!("arm drone");
    client.arm_disarm(true).await?;
    log::info!("Response: {:?}", res);

    log::info!("turn 180 and go to 3m");
    client_clone
        .move_by_roll_pitch_yaw_z_async(Orientation3::new(0.0, 0.0, 1.57), -3.0, 3.0)
        .await
        .unwrap();
    log::info!("done!");

    log::info!("turn 180 and go to 3m");
    client_clone
        .move_by_roll_pitch_yaw_z_async(Orientation3::new(1.2, 0.0, 1.57), -8.0, 3.0)
        .await
        .unwrap();
    log::info!("done!");

    thread::sleep(Duration::from_secs(2));
    log::info!("land drone");
    let landed = client.land_async(20.0).await.unwrap();
    log::info!("drone landed: {}", landed);

    client.arm_disarm(false).await.unwrap();
    client.enable_api_control(false).await.unwrap();
    log::info!("Mission done!");
    Ok(())
}

#[tokio::main]
async fn main() -> NetworkResult<()> {
    env_logger::init();
    // task::block_on(connect_drone())
    connect_drone().await.unwrap();
    Ok(())
}
