use async_std::net::ToSocketAddrs;
use rmp_rpc::{
    message::{Request, Response},
    Utf8String,
};
use rmpv::Value;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::{
    error::NetworkResult, types::geopoint::GeoPoint, CompressedImage, ImageType, MsgPackClient, NetworkError,
    WeatherParameter,
};

pub struct AirsimClient {
    client: MsgPackClient,
    last_request_id: AtomicU32,
}

impl AirsimClient {
    pub async fn new(addrs: impl ToSocketAddrs, vehicle_name: &str) -> NetworkResult<Self> {
        let airsim = Self {
            last_request_id: AtomicU32::new(0),
            client: MsgPackClient::connect(addrs).await?,
        };
        airsim.ping().await?;
        airsim.enable_api_control(true, Some(vehicle_name)).await?;
        Ok(airsim)
    }

    #[allow(deprecated)]
    fn new_request_id(&self) -> u32 {
        self.last_request_id
            // TODO: method below is deprecated
            .compare_and_swap(u32::max_value(), 0, Ordering::AcqRel);
        self.last_request_id.fetch_add(1, Ordering::AcqRel)
    }

    pub(crate) async fn unary_rpc(&self, method: String, params: Option<Vec<Value>>) -> NetworkResult<Response> {
        self.client
            .request(Request {
                id: self.new_request_id(),
                method,
                params: params.unwrap_or_default(),
            })
            .await
            .map_err(Into::into)
    }

    /// Get client version
    fn get_client_version() -> u64 {
        1
    }

    /// Get AirSim server version
    async fn get_server_version(&self) -> NetworkResult<u64> {
        self.unary_rpc("getServerVersion".to_owned(), None)
            .await
            .map(|res| {
                res.result
                    .unwrap_or_else(|_| rmpv::Value::Integer(0.into()))
                    .as_u64()
                    .unwrap_or(0)
            })
            .map_err(Into::into)
    }

    /// Get minimum required client version
    async fn get_min_required_client_version(&self) -> NetworkResult<u64> {
        self.unary_rpc("getMinRequiredClientVersion".to_owned(), None)
            .await
            .map(|res| {
                res.result
                    .unwrap_or_else(|_| rmpv::Value::Integer(0.into()))
                    .as_u64()
                    .unwrap_or(0)
            })
            .map_err(Into::into)
    }

    #[inline]
    fn get_min_required_server_version() -> u64 {
        Self::get_client_version()
    }

    /// Reset the vehicle to its original starting state
    ///
    /// Note that you must call `enable_api_control` and `arm_disarm` again after the call to reset
    pub async fn reset(&self) -> NetworkResult<bool> {
        self.unary_rpc("reset".to_owned(), None)
            .await
            .map(|res| res.result.unwrap_or(rmpv::Value::Nil).is_nil())
            .map_err(Into::into)
    }

    /// If connection is established then this call will return `True` otherwise
    /// the request will be blocked until timeout (default value)
    pub async fn ping(&self) -> NetworkResult<bool> {
        self.unary_rpc("ping".to_owned(), None)
            .await
            .map(|res| {
                res.result
                    .unwrap_or(rmpv::Value::Boolean(false))
                    .as_bool()
                    .unwrap_or(false)
            })
            .map_err(Into::into)
    }

    /// Checks state of the connection
    pub(crate) async fn confirm_connection(&self) -> NetworkResult<bool> {
        let connected = self.ping().await?;

        log::info!("Connected to Airsim: {}", connected);

        let client_v = Self::get_client_version();
        let client_min_v = self.get_min_required_client_version().await?;
        let server_v = self.get_server_version().await?;
        let server_min_v = Self::get_min_required_server_version();

        log::info!("Client version: {} , Min required: {} ", client_v, client_min_v);
        log::info!("Server version: {} , Min required: {} ", server_v, server_min_v);

        if server_v < server_min_v {
            log::error!("AirSim server is of older version and not supported by this client. Please upgrade!")
        } else if client_v < client_min_v {
            log::error!("AirSim client is of older version and not supported by this server. Please upgrade!")
        }

        Ok(connected)
    }

    /// Pauses simulation
    ///
    /// args:
    ///     is_paused (bool): True to pause the simulation, False to release
    pub async fn sim_pause(&self, is_paused: bool) -> NetworkResult<bool> {
        self.unary_rpc("simPause".into(), Some(vec![Value::Boolean(is_paused)]))
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Returns True if simulation is paused
    pub async fn sim_is_pause(&self) -> NetworkResult<bool> {
        self.unary_rpc("simIsPause".into(), None)
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Continue the simulation for the specified number of seconds
    ///
    /// args:
    ///     seconds (f64): Time to run the simulation for
    pub async fn sim_continue_for_time(&self, seconds: f64) -> NetworkResult<()> {
        self.unary_rpc("simContinueFortime".into(), Some(vec![Value::F64(seconds)]))
            .await
            .map_err::<NetworkError, _>(Into::into)
            .map(|_| ())
    }

    /// Continue (or resume if paused) the simulation for the specified number of frames,
    /// after which the simulation will be paused.
    ///
    /// args:
    ///     frames (i64): Frames to run the simulation for
    pub async fn sim_continue_for_frames(&self, frames: i64) -> NetworkResult<()> {
        self.unary_rpc("simContinueFortime".into(), Some(vec![Value::Integer(frames.into())]))
            .await
            .map_err::<NetworkError, _>(Into::into)
            .map(|_| ())
    }

    /// Change intensity of named light
    ///
    /// args:
    ///     light_name (str): Name of light to change
    ///     intensity (f32): New intensity value
    pub async fn sim_set_light_intensity(&self, _light_name: &str, _intensity: f32) -> NetworkResult<bool> {
        unimplemented!("todo")
    }

    /// Runtime swap texture API
    ///
    /// Returns vector of objects which matched the provided tags and had the texture swap perfomed
    /// See https://microsoft.github.io/AirSim/retexturing/ for details
    ///
    /// args:
    ///     tags (str): String of "," or ", " delimited tags to identify on which actors to perform the swap
    ///     tex_id (Option<i32>): Indexes the array of textures assigned to each actor undergoing a swap
    ///     component_id (Option<i32>): Id of the component
    ///     material_id (Option<i32>): Id of the material
    pub async fn sim_swap_textures(
        &self,
        _tags: &str,
        _tex_id: Option<i32>,
        _component_id: Option<i32>,
        _material_id: Option<i32>,
    ) -> NetworkResult<Vec<String>> {
        unimplemented!("todo")
    }

    /// Runtime swap texture API
    ///
    /// Returns True if material was set
    /// See https://microsoft.github.io/AirSim/retexturing/ for details
    ///
    /// args:
    ///     object_name (&str): Name of the object to set material for
    ///     material_name (&str): Name of the material to set for object
    ///     component_id (Option<i32>): Id of the component
    pub async fn sim_set_object_material(
        &self,
        _tags: &str,
        _tex_id: Option<i32>,
        _component_id: Option<i32>,
        _material_id: Option<i32>,
    ) -> NetworkResult<bool> {
        unimplemented!("todo")
    }

    /// Runtime swap texture API
    ///
    /// Returns True if material was set
    /// See https://microsoft.github.io/AirSim/retexturing/ for details
    ///
    /// args:
    ///     object_name (&str): Name of the object to set material for
    ///     material_name (&str): Name of the material to set for object
    ///     component_id (Option<i32>): Id of the component
    pub async fn sim_set_object_material_from_texture(
        &self,
        _tags: &str,
        _tex_id: Option<i32>,
        _component_id: Option<i32>,
        _material_id: Option<i32>,
    ) -> NetworkResult<bool> {
        unimplemented!("todo")
    }

    /// Time API
    ///
    /// Control the position of Sun in the environment
    /// Sun's position is computed using the coordinates specified in `OriginGeopoint` in settings for the date-time specified in the argument,
    /// else if the string is empty, current date & time is used
    ///
    /// args:
    ///    is_enabled (bool): True to enable time-of-day effect, False to reset the position to original
    ///    start_datetime (Option<bool>): Date & Time in %Y-%m-%d %H:%M:%S format, e.g. `2018-02-12 15:20:00`
    ///    is_start_datetime_dst (Option<bool): True to adjust for Daylight Savings Time
    ///    celestial_clock_speed (Option<f32>): Run celestial clock faster or slower than simulation clock
    ///                                             E.g. Value 100 means for every 1 second of simulation clock, Sun's position is advanced by 100 seconds
    ///                                             so Sun will move in sky much faster
    ///    update_interval_secs (Option<f32>): Interval to update the Sun's position
    ///    move_sun (Option<bool>): Whether or not to move the Sun
    pub async fn sim_set_time_of_day(
        &self,
        _is_enabled: bool,
        _start_datetime: &str,
        _is_start_datetime_dst: bool,
        _celestial_clock_speed: f32,
        _update_interval_secs: f32,
        _move_sun: bool,
    ) -> NetworkResult<()> {
        unimplemented!("todo")
    }

    /// Weather API
    ///
    /// Enable Weather effects. Needs to be called before using `sim_set_weather_parameter()` method
    /// args:
    ///     enable (bool): true to enable, false to disable
    pub async fn sim_enable_weather(&self, _enable: bool) -> NetworkResult<()> {
        unimplemented!("todo")
    }

    /// Weather API
    ///
    /// Enable various weather effects
    ///
    /// args:
    ///     param (WeatherParameter): Weather effect to be enabled
    ///     val (f32): Intensity of the effect, Range 0-1
    pub async fn sim_set_weather_parameter(&self, _param: WeatherParameter, val: f32) -> NetworkResult<()> {
        if val.is_sign_negative() || val > 1.0 {
            panic!("val outside of valid range 0.0 to 1.0")
        }

        unimplemented!("todo")
    }
}

/// Vehicle specific functions
impl AirsimClient {
    /// Enables or disables API control for vehicle corresponding to vehicle_name
    ///
    /// args:
    ///     is_enabled (bool): True to enable, False to disable API control
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    pub(crate) async fn enable_api_control(&self, is_enabled: bool, vehicle_name: Option<&str>) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = vehicle_name.unwrap_or("").into();

        self.unary_rpc(
            "enableApiControl".into(),
            Some(vec![Value::Boolean(is_enabled), Value::String(vehicle_name)]),
        )
        .await
        .map_err(Into::into)
        .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    pub(crate) async fn is_api_control_enabled(
        &self,
        is_enabled: bool,
        vehicle_name: Option<&str>,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = vehicle_name.unwrap_or("").into();

        self.unary_rpc(
            "isApiControlEnabled".into(),
            Some(vec![Value::Boolean(is_enabled), Value::String(vehicle_name)]),
        )
        .await
        .map_err(Into::into)
        .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     arm (bool): True to arm, False to disarm the vehicle
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    pub(crate) async fn arm_disarm(&self, arm: bool, vehicle_name: Option<&str>) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = vehicle_name.unwrap_or("").into();

        self.unary_rpc(
            "armDisarm".into(),
            Some(vec![Value::Boolean(arm), Value::String(vehicle_name)]),
        )
        .await
        .map_err(Into::into)
        .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Get the Home location of the vehicle
    ///
    /// args:
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    pub(crate) async fn get_home_geo_point(&self, vehicle_name: Option<&str>) -> Result<GeoPoint, NetworkError> {
        let vehicle_name: Utf8String = vehicle_name.unwrap_or("").into();

        self.unary_rpc("getHomeGeoPoint".into(), Some(vec![Value::String(vehicle_name)]))
            .await
            .map_err(Into::into)
            .map(GeoPoint::from)
    }

    /// Camera API
    ///
    /// Returns binary string literal of compressed png image in presented as an vector of bytes
    ///
    /// Returns bytes of png format image which can be dumped into abinary file to create .png image
    /// See https://microsoft.github.io/AirSim/image_apis/ for details
    ///
    /// args:
    ///     camera_name (String): Name of the camera, for backwards compatibility, ID numbers such as 0,1,etc. can also be used
    ///     image_type (ImageType): Type of image required
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    ///     external (Option<bool>): Whether the camera is an External Camera
    pub(crate) async fn _sim_get_image(
        &self,
        _camera_name: &str,
        _image_type: ImageType,
        _vehicle_name: Option<&str>,
        _external: Option<bool>,
    ) -> Result<CompressedImage, NetworkError> {
        unimplemented!("todo")
    }
}
