use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Orientation3, RCData, Velocity3, YawMode};
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

    //take off
    // log::info!("take off drone");
    // client.take_off_async(10.0).await?;
    // log::info!("take off completed");

    tokio::spawn(async move {
        log::info!("set in manual mode");
        client_clone
            .move_by_manual_async(
                Velocity3::new(200000.0, 200000.0, 200000.0),
                -10000.0,
                15.0,
                DrivetrainType::MaxDegreeOfFreedom,
                YawMode::new(false, 0.0),
            )
            .await
            .unwrap();
    });

    log::info!("move by RC");
    client
        .move_by_rc(RCData::new(0, Orientation3::new(0.0, 0.0, 0.0), 1.0, None, true, true))
        .await?;
    log::info!("done with RC");

    log::info!("sleep for 2 sec");
    thread::sleep(Duration::from_secs(4));

    log::info!("move by RC again");
    client
        .move_by_rc(RCData::new(0, Orientation3::new(0.5, 0.0, 0.5), 0.6, None, true, true))
        .await?;

    log::info!("sleep for 4 sec");
    thread::sleep(Duration::from_secs(4));

    log::info!("move by RC again");
    client
        .move_by_rc(RCData::new(0, Orientation3::new(-0.1, 0.3, 1.5), 3.0, None, true, true))
        .await?;
    log::info!("done with RC");

    thread::sleep(Duration::from_secs(4));
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
