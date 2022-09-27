use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Orientation3, RCData, Velocity3, YawMode, PWM};
use std::{sync::Arc, thread, time::Duration};
// use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.17.176.1:41451"; // set with env variable
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

    log::info!("move by manual pwm");
    client
        .move_by_motor_pwm_async(PWM::new(0.6, 0.6, 0.6, 0.6), 6.0)
        .await?;
    log::info!("done with pwm");

    log::info!("move by pwm again");
    client
        .move_by_motor_pwm_async(PWM::new(0.6, 0.605, 0.6, 0.605), 1.0)
        .await?;
    log::info!("done with pwm");

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
