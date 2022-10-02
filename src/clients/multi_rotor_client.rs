use async_std::net::ToSocketAddrs;
use rmp_rpc::Utf8String;
use rmpv::Value;

use crate::types::drive_train::DrivetrainType;
use crate::types::geopoint::GeoPoint;
use crate::types::pose::{Orientation3, Position3, Velocity3};
use crate::types::pwm::PWM;
use crate::types::rc_data::RCData;
use crate::types::yaw_mode::YawMode;
use crate::{error::NetworkResult, NetworkError};
use crate::{Path, Velocity2};

use super::airsim_client::AirsimClient;

pub struct MultiRotorClient {
    airsim_client: AirsimClient,
    vehicle_name: &'static str,
}

impl MultiRotorClient {
    pub async fn connect(addrs: impl ToSocketAddrs, vehicle_name: &'static str) -> NetworkResult<Self> {
        let airsim_client = AirsimClient::new(addrs, vehicle_name).await?;
        Ok(Self {
            airsim_client,
            vehicle_name,
        })
    }

    /// Reset the vehicle to its original starting state
    ///
    /// Note that you must call `enable_api_control` and `arm_disarm` again after the call to reset
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

    /// Hover the vehicle in place
    pub async fn hover_async(&self) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc("hover".into(), Some(vec![Value::String(vehicle_name)]))
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok())
    }

    /// Get the Home location of the vehicle
    pub async fn get_home_geo_point(&self) -> Result<GeoPoint, NetworkError> {
        self.airsim_client.get_home_geo_point(Some(self.vehicle_name)).await
    }

    /// Takeoff vehicle to 3m above ground. Vehicle should not be moving when this API is used
    ///
    /// Args:
    ///     timeout_sec (Option<f32>): Timeout for the vehicle to reach desired altitude
    pub async fn take_off_async(&self, timeout_sec: f32) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "takeoff".into(),
                Some(vec![Value::F32(timeout_sec), Value::String(vehicle_name)]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Safely land the vehicle in a vertical only movement.
    /// This function should close to the ground
    ///
    /// Args:
    ///     timeout_sec (Option<f32>): Timeout for the vehicle to land
    pub async fn land_async(&self, timeout_sec: f32) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "land".into(),
                Some(vec![Value::F32(timeout_sec), Value::String(vehicle_name)]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Return vehicle to Home i.e. Launch location
    /// The vehicle should be in the viscinity of home when this function
    /// is called
    ///
    /// Args:
    ///     timeout_sec (Option<f32>): Timeout for the vehicle to reach desired altitude
    pub async fn go_home_async(&self, timeout_sec: f32) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "goHome".into(),
                Some(vec![Value::F32(timeout_sec), Value::String(vehicle_name)]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set 3D velocity vector in vehicle's local NED frame
    ///
    /// Args:
    ///     velocity (Velocity3): desired velocity in the X,Y,Z axis's of the vehicle's local NED frame.
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    pub async fn move_by_velocity_body_frame_async(
        &self,
        velocity: Velocity3,
        duration: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByVelocityBodyFrame".into(),
                Some(vec![
                    rmp_rpc::Value::F32(velocity.vx),
                    rmp_rpc::Value::F32(velocity.vy),
                    rmp_rpc::Value::F32(velocity.vz),
                    rmp_rpc::Value::F32(duration),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set 2D velocity vector in vehicle's local NED frame, with desired Z altitude.
    ///
    /// Args:
    ///     velocity (Velocity2): desired velocity in the X,Y axis's of the vehicle's local NED frame.
    ///     z (f32): desired Z value (in local NED frame of the vehicle)
    ///     duration (f32): desired amount of time (seconds), to send this command for
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    pub async fn move_by_velocity_z_body_frame_async(
        &self,
        velocity: Velocity2,
        z: f32,
        duration: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByVelocityZBodyFrame".into(),
                Some(vec![
                    rmp_rpc::Value::F32(velocity.vx),
                    rmp_rpc::Value::F32(velocity.vy),
                    rmp_rpc::Value::F32(z),
                    rmp_rpc::Value::F32(duration),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set 3D velocity vector in vehicle's local NED frame
    ///
    /// Args:
    ///     velocity (Velocity3): desired velocity X,Y,Z in world (NED) axis
    ///     duration (f32): desired amount of time (seconds), to send this command for
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    pub async fn move_by_velocity_async(
        &self,
        velocity: Velocity3,
        duration: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByVelocity".into(),
                Some(vec![
                    rmp_rpc::Value::F32(velocity.vx),
                    rmp_rpc::Value::F32(velocity.vy),
                    rmp_rpc::Value::F32(velocity.vz),
                    rmp_rpc::Value::F32(duration),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set 2D velocity vector in vehicle's local NED frame, with desired Z attitude.
    ///
    /// Args:
    ///     velocity (Velocity2): desired velocity in the X,Y axis's of the vehicle's local NED frame.
    ///     z (f32): desired Z value (in local NED frame of the vehicle)
    ///     duration (f32): desired amount of time (seconds), to send this command for
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    pub async fn move_by_velocity_z_async(
        &self,
        velocity: Velocity2,
        z: f32,
        duration: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByVelocityZ".into(),
                Some(vec![
                    rmp_rpc::Value::F32(velocity.vx),
                    rmp_rpc::Value::F32(velocity.vy),
                    rmp_rpc::Value::F32(z),
                    rmp_rpc::Value::F32(duration),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Send desired goal position to default PID vehicle controller
    ///
    /// Args:
    ///     position (Position3): goal position of the vehicle controller
    ///     velocity (f32): desired velocity in NED frame of the vehicle
    ///     timeout_sec (32): Timeout for the vehicle to reach desired goal position
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    ///     lookahead (Option<i32>): defaults to `-1`
    ///     adaptive_lookahead (Option<i32>): defaults to `0`
    #[allow(clippy::too_many_arguments)]
    pub async fn move_to_position_async(
        &self,
        position: Position3,
        velocity: f32,
        timeout_sec: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
        lookahead: Option<f32>,
        adaptive_lookahead: Option<f32>,
    ) -> NetworkResult<bool> {
        let lookahead = lookahead.unwrap_or(-1.0);
        let adaptive_lookahead = adaptive_lookahead.unwrap_or(1.0);
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveToPosition".into(),
                Some(vec![
                    rmp_rpc::Value::F32(position.x),
                    rmp_rpc::Value::F32(position.y),
                    rmp_rpc::Value::F32(position.z),
                    rmp_rpc::Value::F32(velocity),
                    rmp_rpc::Value::F32(timeout_sec),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    rmp_rpc::Value::F32(lookahead),
                    rmp_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Send desired goal position to default PID vehicle controller
    ///
    /// Args:
    ///     position (Position3): goal position of the vehicle controller
    ///     velocity (f32): desired velocity in NED frame of the vehicle
    ///     timeout_sec (32): Timeout for the vehicle to reach desired goal position
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    ///     lookahead (Option<i32>): defaults to `-1`
    ///     adaptive_lookahead (Option<i32>): defaults to `0`
    #[allow(clippy::too_many_arguments)]
    pub async fn move_on_path_async(
        &self,
        path: Path,
        velocity: f32,
        timeout_sec: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
        lookahead: Option<f32>,
        adaptive_lookahead: Option<f32>,
    ) -> NetworkResult<bool> {
        let lookahead = lookahead.unwrap_or(-1.0);
        let adaptive_lookahead = adaptive_lookahead.unwrap_or(1.0);
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveOnPath".into(),
                Some(vec![
                    path.to_msgpack(),
                    rmp_rpc::Value::F32(velocity),
                    rmp_rpc::Value::F32(timeout_sec),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    rmp_rpc::Value::F32(lookahead),
                    rmp_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Send desired goal position to default PID vehicle controller
    ///
    /// Args:
    ///     position (Position3): goal position of the vehicle controller
    ///     velocity (f32): desired velocity in NED frame of the vehicle
    ///     timeout_sec (32): Timeout for the vehicle to reach desired goal position
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    ///     lookahead (Option<i32>): defaults to `-1`
    ///     adaptive_lookahead (Option<i32>): defaults to `0`
    #[allow(clippy::too_many_arguments)]
    pub async fn move_to_gps_async(
        &self,
        geopoint: GeoPoint,
        velocity: f32,
        timeout_sec: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
        lookahead: Option<f32>,
        adaptive_lookahead: Option<f32>,
    ) -> NetworkResult<bool> {
        let lookahead = lookahead.unwrap_or(-1.0);
        let adaptive_lookahead = adaptive_lookahead.unwrap_or(1.0);
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveToGPS".into(),
                Some(vec![
                    rmp_rpc::Value::F32(geopoint.latitude),
                    rmp_rpc::Value::F32(geopoint.longitude),
                    rmp_rpc::Value::F32(geopoint.altitude),
                    rmp_rpc::Value::F32(velocity),
                    rmp_rpc::Value::F32(timeout_sec),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    rmp_rpc::Value::F32(lookahead),
                    rmp_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Move to a desired altitude Z (in local NED frame of the vehicle) with a desired velocity
    ///
    /// Args:
    ///     z (f32): desired Z value (in local NED frame of the vehicle)
    ///     velocity (f32): desired velocity in NED frame of the vehicle
    ///     timeout_sec (32): Timeout for the vehicle to reach desired goal altitude Z
    ///     yaw_mode (YawMode, Degree): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    ///     lookahead (Option<i32>): defaults to `-1`
    ///     adaptive_lookahead (Option<i32>): defaults to `0`
    #[allow(clippy::too_many_arguments)]
    pub async fn move_to_z_async(
        &self,
        z: f32,
        velocity: f32,
        timeout_sec: f32,
        yaw_mode: YawMode,
        lookahead: Option<f32>,
        adaptive_lookahead: Option<f32>,
    ) -> NetworkResult<bool> {
        let lookahead = lookahead.unwrap_or(-1.0);
        let adaptive_lookahead = adaptive_lookahead.unwrap_or(1.0);
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveToZ".into(),
                Some(vec![
                    rmp_rpc::Value::F32(z),
                    rmp_rpc::Value::F32(velocity),
                    rmp_rpc::Value::F32(timeout_sec),
                    yaw_mode.to_msgpack(),
                    rmp_rpc::Value::F32(lookahead),
                    rmp_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set the vehicle in a manual mode state.
    /// Parameters sets up the constraints on velocity and minimum altitude while flying.
    /// If RC state is detected to violate these constraintsthen that RC state would be ignored.
    ///
    /// Call this method followed by `move_by_rc` method to remote control the vehicle
    ///
    /// Args:
    ///     v_max (Velocity3): max velocity allowed in X, Y, Z direction
    ///     z_min (f32): min Z (altitude) allowed for vehicle position
    ///     duration (f32): after this duration vehicle would switch back to non-manual mode
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode, Degree): specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    pub async fn move_by_manual_async(
        &self,
        v_max: Velocity3,
        z_min: f32,
        duration: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByManual".into(),
                Some(vec![
                    rmp_rpc::Value::F32(v_max.vx),
                    rmp_rpc::Value::F32(v_max.vy),
                    rmp_rpc::Value::F32(z_min),
                    rmp_rpc::Value::F32(duration),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Remote control the robot in joystick mode
    ///
    /// args:
    ///     rc_data (RCData): remote control commands
    pub async fn move_by_rc(&self, rc_data: RCData) -> NetworkResult<()> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByRC".into(),
                Some(vec![rc_data.to_msgpack(), Value::String(vehicle_name)]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.unwrap())
            .map(|value| {
                if value.is_nil() {
                    ()
                } else {
                    panic!("Value {} is not Nil", value)
                }
            })
    }

    /// Low level control API
    ///
    /// Directly control the motors using PWM values
    /// convert thrust to pwm: https://github.com/microsoft/AirSim/issues/2592
    ///
    /// args:
    ///     pwm (PWM): pwm signals for each indivual rotor (4 rotors in total)
    ///     duration (f32): desired amount of time (seconds), to send this command for
    pub async fn move_by_motor_pwms_async(&self, pwm: PWM, duration: f32) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByMotorPWMs".into(),
                Some(vec![
                    rmp_rpc::Value::F32(pwm.front_right_pwm),
                    rmp_rpc::Value::F32(pwm.rear_left_pwm),
                    rmp_rpc::Value::F32(pwm.front_left_pwm),
                    rmp_rpc::Value::F32(pwm.rear_right_pwm),
                    rmp_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Directly control the motors using PWM values
    /// convert thrust to pwm: https://github.com/microsoft/AirSim/issues/2592
    ///
    /// args:
    ///     rotation (Orientation3): Roll angle, pitch angle, and yaw angle set points are given in **radians**, in the ENU body frame.
    ///     z (f32): altitude z is given in local NED frame of the vehicle.
    ///     duration (f32): desired amount of time (seconds), to send this command for
    pub async fn move_by_roll_pitch_yaw_z_async(
        &self,
        rotation: Orientation3,
        z: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByRollPitchYawZ".into(),
                Some(vec![
                    rmp_rpc::Value::F32(rotation.roll),
                    rmp_rpc::Value::F32(-rotation.pitch),
                    rmp_rpc::Value::F32(-rotation.yaw),
                    rmp_rpc::Value::F32(z),
                    rmp_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }
}
