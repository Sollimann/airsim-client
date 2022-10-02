use std::time::Duration;

use airsim_client::{AngularControllerGains, LinearControllerGains, MultiRotorClient, NetworkResult, PIDGains};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.21.176.1:41451";
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

    // set control gains
    log::info!("set angle rate controller PID gains");
    let pid_gains = PIDGains::new(2.5, 0.0, 0.0);
    let res = client
        .set_angle_rate_controller_gains(AngularControllerGains::new(
            pid_gains.to_owned(),
            pid_gains.to_owned(),
            pid_gains.to_owned(),
        ))
        .await?;
    log::info!("Response: {:?}", res);

    log::info!("set angle level controller PID gains");
    let res = client
        .set_angle_level_controller_gains(AngularControllerGains::new(
            pid_gains.to_owned(),
            pid_gains.to_owned(),
            pid_gains.to_owned(),
        ))
        .await?;
    log::info!("Response: {:?}", res);

    log::info!("set velocity controller PID gains");
    let res = client
        .set_velocity_controller_gains(LinearControllerGains::new(
            pid_gains.to_owned(),
            pid_gains.to_owned(),
            pid_gains.to_owned(),
        ))
        .await?;
    log::info!("Response: {:?}", res);

    log::info!("set position controller PID gains");
    let res = client
        .set_position_controller_gains(LinearControllerGains::new(
            pid_gains.to_owned(),
            pid_gains.to_owned(),
            pid_gains.to_owned(),
        ))
        .await?;
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
