<h1 align="center" font-size:4em;"> Rust Airsim </h1>
<p align="center">
  <img src="https://github.com/Sollimann/airsim-client/blob/main/docs/drone_lobby.png" width="750">
</p>

<p align="center">
    <em> A Rust Client library for Microsoft Airsim</em>
</p>

<p align="center">
    <em> Contributions are welcome! </em>
</p>

[![Build Status](https://github.com/Sollimann/airsim-client/actions/workflows/rust-ci.yml/badge.svg?event=push)](https://github.com/Sollimann/airsim-client/actions)
[![airsim-client crate](https://img.shields.io/crates/v/airsim-client.svg)](https://crates.io/crates/airsim-client)
[![minimum rustc 1.56](https://img.shields.io/badge/rustc-1.56+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![Docs](https://docs.rs/airsim-client/badge.svg)](https://docs.rs/airsim-client)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Sollimann/airsim-client/graphs/commit-activity)
[![GitHub pull-requests](https://img.shields.io/github/issues-pr/Sollimann/airsim-client.svg)](https://GitHub.com/Sollimann/airsim-client/pulls)
[![GitHub pull-requests closed](https://img.shields.io/github/issues-pr-closed/Sollimann/airsim-client.svg)](https://GitHub.com/Sollimann/airsim-client/pulls)
![ViewCount](https://views.whatilearened.today/views/github/Sollimann/airsim-client.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## How to use

See [examples](examples/) folder for more!

Use e.g this [settings.json](examples/multirotor/settings.json) to configure multirotor in Airsim

Once you have Airsim up and running (some examples below on how run), execute the snippet:

```rust
use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Path, Vector3, YawMode};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "<Airsim IP here>:41451"; // set with env variable
    let vehicle_name = ""; // use default vehicle name
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
    client.take_off_async(20.0).await?;
    log::info!("take off completed");

    log::info!("move on path");
    client
        .move_on_path_async(
            Path(vec![
                Vector3::new(-25.0, 0.0, -20.0),
                Vector3::new(-50.0, 50.0, -20.0),
                Vector3::new(-50.0, -50.0, -25.0),
            ]),
            5.0,
            1000.0,
            DrivetrainType::MaxDegreeOfFreedom,
            YawMode::new(false, 90.0),
            None,
            None,
        )
        .await?;
    log::info!("done!");

    log::info!("go home");
    client.go_home_async(20.0).await?;
    log::info!("got home");

    log::info!("land drone");
    let landed = client.land_async(20.0).await?;
    log::info!("drone landed: {landed}");

    log::info("Disarm drone")
    client.arm_disarm(false).await?;
    client.enable_api_control(false).await?;
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
```

## Pre-requisites to build project

```sh
$ sudo apt install build-ess    ential
```

## Running with Docker

Install `docker-nvidia` first

then

```sh
$ make airsim-up
```

## WSL2 (client) and Windows 11 Server

https://docs.microsoft.com/en-us/windows/wsl/networking

Open Windows PowerShell in directory `Blocks\Blocks\WindowsNoEditor` and type:

```PowerShell
./Blocks.exe -ResX=640 -ResY=480 -windowed
```
