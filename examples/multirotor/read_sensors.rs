use airsim_client::{MultiRotorClient, NetworkResult};
use async_std::task;
use msgpack_rpc::Client;
use tokio::net::{unix::SocketAddr, TcpStream};
use tokio_util::compat::TokioAsyncReadCompatExt;

#[allow(clippy::no_effect)]
fn _settings_json() {
    r#"
    "#;
}

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.22.224.1:41451"; // set with env variable
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");

    // Create a future that connects to the server, and send a notification and a request.
    let socket = TcpStream::connect(&address).await.unwrap();
    let client = Client::new(socket.compat());

    let resp = client.request("getLidarData", &["Lidar2".into(), "".into()]).await;
    // println!("resp {resp:?}");
    log::info!("Mission done!");
    Ok(())
}
#[tokio::main]
async fn main() -> NetworkResult<()> {
    env_logger::init();
    connect_drone().await.unwrap();
    Ok(())
}
