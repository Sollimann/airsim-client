use std::time::Duration;

use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Position3, YawMode};
use async_std::task;

use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
};

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.21.112.1:41451"; // set with env variable
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

    // take off
    log::info!("take off drone");
    let _t1 = client.take_off_async(20.0).fuse().await?;
    let _t2 = task::sleep(Duration::from_secs(10)).fuse();

    pin_mut!(_t1, _t2);

    log::info!("get home geo point");
    let geopoint = client.get_home_geo_point().await.unwrap();
    println!("geopoint: {:?}", geopoint);

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
    client
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
    log::info!("finished going to geopoint");

    log::info!("go home");
    client.go_home_async(20.0).await?;
    log::info!("got home");

    log::info!("land drone");
    let landed = client.land_async(20.0).await?;
    log::info!("drone landed: {}", landed);

    client.arm_disarm(false).await?;
    client.enable_api_control(false).await?;
    log::info!("Mission done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
