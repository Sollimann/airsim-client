use std::time::Duration;

use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Position3, YawMode};
use async_std::task;

use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
};

async fn connect_drone() -> NetworkResult<()> {
    let address = "127.0.0.1:41451"; // set with env variable
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let mut res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // arm drone
    log::info!("arm drone");
    res = client.arm_disarm(true).await?;
    log::info!("Response: {:?}", res);

    // take off
    log::info!("take off drone");
    let _t1 = client.take_off_async(20.0).fuse().await?;
    let _t2 = task::sleep(Duration::from_secs(10)).fuse();

    pin_mut!(_t1, _t2);

    log::info!("get home geo point");
    let geopoint = client.get_home_geo_point().await.unwrap();
    println!("Geopoint: {geopoint:?}");

    // OPTIONAL - set gains
    // log::info!("set position controller PID gains");
    // let pid_gains = PIDGains::new(2.5, 0.0, 0.0);
    // let res = client
    //     .set_position_controller_gains(LinearControllerGains::new(
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //         pid_gains.to_owned(),
    //     ))
    //     .await?;
    // log::info!("Response: {:?}", res);

    log::info!("move to position");
    client
        .move_to_position_async(
            Position3::new(-10.0, 10.0, -30.0),
            3.0,
            1000.0,
            DrivetrainType::ForwardOnly,
            YawMode::new(false, 90.0),
            None,
            None,
        )
        .await?;

    client
        .move_to_position_async(
            Position3::new(-30.0, 70.0, -25.0),
            7.0,
            1000.0,
            DrivetrainType::ForwardOnly,
            YawMode::new(false, 180.0),
            None,
            None,
        )
        .await?;

    log::info!("go to geopoint");
    res = client
        .move_to_gps_async(
            geopoint,
            6.0,
            1000.0,
            DrivetrainType::ForwardOnly,
            YawMode::new(false, 70.0),
            None,
            None,
        )
        .await?;
    log::info!("Finished going to geopoint: {res:?}");

    log::info!("go home");
    res = client.go_home_async(20.0).await?;
    log::info!("Got home: {:?}", res);

    log::info!("land drone");
    res = client.land_async(20.0).await?;
    log::info!("Drone landed: {res:?}");

    client.arm_disarm(false).await?;
    client.enable_api_control(false).await?;
    log::info!("Mission done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
