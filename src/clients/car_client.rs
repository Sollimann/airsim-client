use async_std::net::ToSocketAddrs;
use rmp_rpc::Utf8String;
use rmpv::Value;

use crate::error::NetworkResult;

use super::airsim_client::AirsimClient;

pub struct CarClient {
    airsim_client: AirsimClient,
    vehicle_name: &'static str,
}

impl CarClient {
    pub async fn connect(addrs: impl ToSocketAddrs, vehicle_name: &'static str) -> NetworkResult<Self> {
        let airsim_client = AirsimClient::connect(addrs, vehicle_name).await?;
        Ok(Self {
            airsim_client,
            vehicle_name,
        })
    }

    /// Reset the vehicle to its original starting state
    ///
    /// Note that you must call `enable_ap, Some(vehicle_name)i_control` and `arm_disarm` again after the call to reset
    #[inline(always)]
    pub async fn reset(&self) -> NetworkResult<bool> {
        self.airsim_client.reset().await
    }

    /// If connection is established then this call will return `True` otherwise
    /// the request will be blocked until timeout (default value)
    #[inline(always)]
    pub async fn ping(&self) -> NetworkResult<bool> {
        self.airsim_client.ping().await
    }

    #[inline(always)]
    pub async fn confirm_connection(&self) -> NetworkResult<bool> {
        self.airsim_client.confirm_connection().await
    }

    /// Enables or disables API control for vehicle corresponding to vehicle_name
    ///
    /// args:
    ///     is_enabled (bool): True to enable, False to disable API control
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn enable_api_control(&self, is_enabled: bool) -> NetworkResult<bool> {
        self.airsim_client
            .enable_api_control(is_enabled, Some(self.vehicle_name))
            .await
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn is_api_control_enabled(&self, is_enabled: bool) -> NetworkResult<bool> {
        self.airsim_client
            .is_api_control_enabled(is_enabled, Some(self.vehicle_name))
            .await
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     arm (bool): True to arm, False to disarm the vehicle
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn arm_disarm(&self, arm: bool) -> NetworkResult<bool> {
        self.airsim_client.arm_disarm(arm, Some(self.vehicle_name)).await
    }

    /// Takeoff vehicle to 3m above ground. Vehicle should not be moving when this API is used
    ///
    /// Args:
    ///     timeout_sec (Option<u64>): Timeout for the vehicle to reach desired altitude
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    pub async fn take_off_async(&self, timeout_sec: u64) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "takeoff".into(),
                Some(vec![Value::Integer(timeout_sec.into()), Value::String(vehicle_name)]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }
}
