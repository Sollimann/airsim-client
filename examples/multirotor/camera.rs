use airsim_client::{ImageType, MultiRotorClient, NetworkResult};
use async_std::task;

#[allow(clippy::no_effect)]
fn _settings_json() {
    r#"
        {
            "SeeDocsAt": "https://github.com/Microsoft/AirSim/blob/main/docs/settings_json.md",
            "SettingsVersion": 1.2,
            "SimMode": "Multirotor",
            "LocalHostIp": "0.0.0.0",
            "ApiServerPort": 41451,
            "Vehicles": {
                "Drone1": {
                    "VehicleType": "SimpleFlight",
                    "AutoCreate": true,
                    "Cameras": {
                        "high_res": {
                            "CaptureSettings": [
                                {
                                    "ImageType": 0,
                                    "Width": 4320,
                                    "Height": 2160
                                }
                            ],
                            "X": 0.50,
                            "Y": 0.00,
                            "Z": 0.10,
                            "Pitch": 0.0,
                            "Roll": 0.0,
                            "Yaw": 0.0
                        },
                        "low_res": {
                            "CaptureSettings": [
                                {
                                    "ImageType": 0,
                                    "Width": 256,
                                    "Height": 144
                                }
                            ],
                            "X": 0.50,
                            "Y": 0.00,
                            "Z": 0.10,
                            "Pitch": 0.0,
                            "Roll": 0.0,
                            "Yaw": 0.0
                        }
                    }
                }
            }
        }
    "#;
}

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.22.224.1:41451"; // set with env variable
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // // arm drone
    // log::info!("arm drone");
    // client.arm_disarm(true).await?;
    // log::info!("Response: {:?}", res);

    // // take off
    // log::info!("take off drone");
    // client.take_off_async(20.0).await?;
    // log::info!("take off completed");

    // use camera
    log::info!("get vehicle image");
    client.sim_get_image("high_res", ImageType::Scene, Some(false)).await?;
    // log::info!("image response: {}");

    client.arm_disarm(false).await?;
    client.enable_api_control(false).await?;
    log::info!("Mission done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
