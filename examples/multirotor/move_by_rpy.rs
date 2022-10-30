use airsim_client::{MultiRotorClient, NetworkResult, Orientation2, Orientation3};
use std::sync::Arc;
// use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.22.224.1:41451"; // set with env variable
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

    // OPTIONAL - set gains
    // log::info!("set angle rate controller PID gains");
    // let pid_gains = PIDGains::new(2.5, 0.0, 0.0);
    // let res = client
    //     .set_angle_rate_controller_gains(AngularControllerGains::new(
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //     ))
    //     .await?;
    // log::info!("Response: {:?}", res);

    // log::info!("set angle level controller PID gains");
    // let res = client
    //     .set_angle_level_controller_gains(AngularControllerGains::new(
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //     ))
    //     .await?;
    // log::info!("Response: {:?}", res);

    log::info!("turn 180 and go to 3m");
    client_clone
        .move_by_roll_pitch_yaw_z_async(Orientation3::new(0.0, 0.0, 1.57), -3.0, 3.0)
        .await
        .unwrap();
    log::info!("done!");

    log::info!("turn negative -180 with throttle");
    client_clone
        .move_by_roll_pitch_yaw_throttle_async(Orientation3::new(0.0, 0.0, -1.57), 0.7, 3.0)
        .await
        .unwrap();
    log::info!("done!");

    log::info!("turn with yawrate and Z throttle");
    let s = client_clone
        .move_by_roll_pitch_yawrate_throttle_async(Orientation2::new(0.0, 0.0), 6.0, 0.45, 2.0)
        .await
        .unwrap();
    log::info!("done! {s:?}");

    log::info!("turn with yawrate and altitude");
    let s = client_clone
        .move_by_roll_pitch_yawrate_z_async(Orientation2::new(0.0, 0.0), 1.0, -10.0, 4.0)
        .await
        .unwrap();
    log::info!("done! {s:?}");

    log::info!("move by angle rates and throttle");
    let s = client_clone
        .move_by_angle_rates_throttle_async(Orientation3::new(0.2, 0.0, 0.0), 0.65, 3.0)
        .await
        .unwrap();
    log::info!("done! {s:?}");

    log::info!("move by angle rates");
    let s = client_clone
        .move_by_angle_rates_z_async(Orientation3::new(-0.4, 0.0, 0.0), -3.0, 3.0)
        .await
        .unwrap();
    log::info!("done! {s:?}");

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
    connect_drone().await.unwrap();
    Ok(())
}
