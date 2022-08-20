use async_std::{channel::RecvError, net::ToSocketAddrs};
use rmp_rpc::message::{Notification, Request, Response};
use rmpv::Value;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::{error::NetworkResult, MsgPackClient};

pub struct MultiRotorClient {
    client: MsgPackClient,
    last_request_id: AtomicU32,
}

impl MultiRotorClient {
    pub async fn connect(addrs: impl ToSocketAddrs) -> NetworkResult<Self> {
        let drone = Self {
            last_request_id: AtomicU32::new(0),
            client: MsgPackClient::connect(addrs).await?,
        };
        drone.ping().await?;
        // drone.enable_api_control().await?;
        Ok(drone)
    }

    async fn unary_rpc(&self, method: String, params: Option<Vec<Value>>) -> NetworkResult<Response> {
        self.client
            .request(Request {
                id: self.new_request_id(),
                method: method,
                params: params.unwrap_or(Vec::new()),
            })
            .await
            .map_err(Into::into)
    }

    pub async fn reset(&self) -> NetworkResult<Response> {
        self.unary_rpc("reset".to_owned(), None).await
    }

    pub async fn ping(&self) -> NetworkResult<Response> {
        self.unary_rpc("ping".to_owned(), None).await
    }

    ///
    /// Checks state of the connection
    ///
    pub fn confirm_connection(&self) {}

    #[allow(deprecated)]
    fn new_request_id(&self) -> u32 {
        self.last_request_id
            // TODO: method below is deprecated
            .compare_and_swap(u32::max_value(), 0, Ordering::AcqRel);
        self.last_request_id.fetch_add(1, Ordering::AcqRel)
    }
}
