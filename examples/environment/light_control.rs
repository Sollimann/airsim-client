use std::{thread, time::Duration};

use airsim_client::{AirsimClient, NetworkResult};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.22.224.1:41451";
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = AirsimClient::connect(address, vehicle_name).await?;

    // confirm connection
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // access an existing light in the world
    log::info!("get existing lights");
    let lights = client.sim_list_scene_objects("PointLight.*").await?;
    log::info!("lights: {lights:?}");

    // get pose of object in the scene
    log::info!("get pose of object in the scene");
    let object_in_scene = lights.0.first().unwrap().to_owned();
    let pose = client.sim_get_object_pose(&object_in_scene).await?;
    log::info!("pose of object: {object_in_scene} is {pose:?}");

    // change light intensity
    log::info!("change light intensity");
    for i in 0..20 {
        let res = client
            .sim_set_light_intensity("trashcan_smallOffice54_107", (i as f32) * 10.0)
            .await?;
        log::info!("Response: {:?}", res);
        thread::sleep(Duration::from_secs(1));
    }

    log::info!("Done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
