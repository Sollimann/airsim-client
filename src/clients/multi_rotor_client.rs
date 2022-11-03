use core::panic;

use msgpack_rpc::Utf8String;
use rmpv::Value;

use crate::types::drive_train::DrivetrainType;
use crate::types::gains::AngularControllerGains;
use crate::types::geopoint::GeoPoint;
use crate::types::image::ImageRequests;
use crate::types::multi_rotor_state::MultiRotorState;
use crate::types::pose::{Orientation2, Orientation3, Position3, Velocity3};
use crate::types::pwm::PWM;
use crate::types::rc_data::RCData;
use crate::types::yaw_mode::YawMode;
use crate::{error::NetworkResult, NetworkError};
use crate::{CompressedImage, ImageType, LinearControllerGains, Path, RotorStates, Velocity2};

use super::airsim_client::AirsimClient;

pub struct MultiRotorClient {
    airsim_client: AirsimClient,
    vehicle_name: &'static str,
}

impl MultiRotorClient {
    pub async fn connect(addrs: &str, vehicle_name: &'static str) -> NetworkResult<Self> {
        let airsim_client = AirsimClient::connect(addrs, vehicle_name).await?;
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

    /// High level control API
    ///
    /// Hover the vehicle in place
    pub async fn hover_async(&self) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc("hover".into(), Some(vec![Value::String(vehicle_name)]))
            .await
            .map(|response| response.result.is_ok())
    }

    /// Get the Home location of the vehicle
    pub async fn get_home_geo_point(&self) -> Result<GeoPoint, NetworkError> {
        self.airsim_client.get_home_geo_point(Some(self.vehicle_name)).await
    }

    /// High level control API
    ///
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
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(velocity.vx),
                    msgpack_rpc::Value::F32(velocity.vy),
                    msgpack_rpc::Value::F32(velocity.vz),
                    msgpack_rpc::Value::F32(duration),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(velocity.vx),
                    msgpack_rpc::Value::F32(velocity.vy),
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(duration),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set PID gains for the velocity controller, move_by_velocity_async().
    ///
    /// - Sets velocity controller gains for moveByVelocityAsync().
    /// - This function should only be called if the default velocity control PID gains need to be modified.
    /// - Passing VelocityControllerGains() sets gains to default airsim values.
    ///
    /// args:
    ///     velocity_gains (LinearControllerGains):
    ///         - Correspond to the world X, Y, Z axes.
    ///         - Pass LinearControllerGains() to reset gains to default recommended values.
    ///         - Modifying velocity controller gains will have an affect on the behaviour of move_on_spline_async() and
    ///           move_on_spline_vel_constraints_async(), as they both use velocity control to track the trajectory.
    pub async fn set_velocity_controller_gains(&self, velocity_gains: LinearControllerGains) -> NetworkResult<bool> {
        self.airsim_client
            .unary_rpc(
                "setVelocityControllerGains".into(),
                Some(velocity_gains.as_msgpack(self.vehicle_name)),
            )
            .await
            .map(|response| response.result.is_ok())
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(velocity.vx),
                    msgpack_rpc::Value::F32(velocity.vy),
                    msgpack_rpc::Value::F32(velocity.vz),
                    msgpack_rpc::Value::F32(duration),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(velocity.vx),
                    msgpack_rpc::Value::F32(velocity.vy),
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(duration),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set PID gains for the position controller, move_to_position_async()
    ///
    /// This function should only be called if the default position control PID gains need to be modified.
    ///
    /// args:
    ///     position_gains (LinearControllerGains):
    ///         - Correspond to the X, Y, Z axes.
    ///         - Pass PositionControllerGains() to reset gains to default recommended values.
    pub async fn set_position_controller_gains(&self, position_gains: LinearControllerGains) -> NetworkResult<bool> {
        self.airsim_client
            .unary_rpc(
                "setPositionControllerGains".into(),
                Some(position_gains.as_msgpack(self.vehicle_name)),
            )
            .await
            .map(|response| response.result.is_ok())
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(position.x),
                    msgpack_rpc::Value::F32(position.y),
                    msgpack_rpc::Value::F32(position.z),
                    msgpack_rpc::Value::F32(velocity),
                    msgpack_rpc::Value::F32(timeout_sec),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    msgpack_rpc::Value::F32(lookahead),
                    msgpack_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    path.as_msgpack(),
                    msgpack_rpc::Value::F32(velocity),
                    msgpack_rpc::Value::F32(timeout_sec),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    msgpack_rpc::Value::F32(lookahead),
                    msgpack_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(geopoint.latitude),
                    msgpack_rpc::Value::F32(geopoint.longitude),
                    msgpack_rpc::Value::F32(geopoint.altitude),
                    msgpack_rpc::Value::F32(velocity),
                    msgpack_rpc::Value::F32(timeout_sec),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    msgpack_rpc::Value::F32(lookahead),
                    msgpack_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// High level control API
    ///
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
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(velocity),
                    msgpack_rpc::Value::F32(timeout_sec),
                    yaw_mode.as_msgpack(),
                    msgpack_rpc::Value::F32(lookahead),
                    msgpack_rpc::Value::F32(adaptive_lookahead),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
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
                    msgpack_rpc::Value::F32(v_max.vx),
                    msgpack_rpc::Value::F32(v_max.vy),
                    msgpack_rpc::Value::F32(z_min),
                    msgpack_rpc::Value::F32(duration),
                    drivetrain.as_msgpack(),
                    yaw_mode.as_msgpack(),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Remote control the robot in joystick mode
    ///
    /// args:
    ///     rc_data (RCData): remote control commands
    pub async fn move_by_rc(&self, rc_data: RCData) -> NetworkResult<()> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByRC".into(),
                Some(vec![rc_data.as_msgpack(), Value::String(vehicle_name)]),
            )
            .await
            .map(|response| response.result.unwrap())
            .map(|value| {
                if !value.is_nil() {
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
                    msgpack_rpc::Value::F32(pwm.front_right_pwm),
                    msgpack_rpc::Value::F32(pwm.rear_left_pwm),
                    msgpack_rpc::Value::F32(pwm.front_left_pwm),
                    msgpack_rpc::Value::F32(pwm.rear_right_pwm),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Set PID gains for the angle rate controller
    ///
    /// - Modifying these gains will have an affect on *ALL* move*() APIs.
    ///     This is because any velocity setpoint is converted to an angle level setpoint which is tracked with an angle level controllers.
    ///     That angle level setpoint is itself tracked with and angle rate controller.
    /// - This function should only be called if the default angle rate control PID gains need to be modified.
    ///
    /// args:
    ///     angle_rate_gains (AngularControllerGains):
    ///         - Correspond to the roll, pitch, yaw axes, defined in the body frame.
    ///         - Pass AngularControllerGains() to reset gains to default recommended values.
    pub async fn set_angle_rate_controller_gains(
        &self,
        angle_rate_gains: AngularControllerGains,
    ) -> NetworkResult<bool> {
        self.airsim_client
            .unary_rpc(
                "setAngleRateControllerGains".into(),
                Some(angle_rate_gains.as_msgpack(self.vehicle_name)),
            )
            .await
            .map(|response| response.result.is_ok())
    }

    /// Set PID gains for the angle level controller
    ///
    /// - Sets angle level controller gains (used by any API setting angle references - for ex: move_by_roll_pitch_yaw_z_async(),
    ///   move_by_roll_pitch_yaw_throttle_async(), etc)
    /// - Modifying these gains will also affect the behaviour of move_by_velocity_async() API.
    ///     This is because the AirSim flight controller will track velocity setpoints by converting them to angle set points.
    /// - This function should only be called if the default angle level control PID gains need to be modified.
    /// - Passing AngularControllerGains() sets gains to default airsim values.
    ///
    /// args:
    ///     angle_level_gains (AngularControllerGains):
    ///         - Correspond to the roll, pitch, yaw axes, defined in the body frame.
    ///         - Pass AngleLevelControllerGains() to reset gains to default recommended values.
    pub async fn set_angle_level_controller_gains(
        &self,
        angle_level_gains: AngularControllerGains,
    ) -> NetworkResult<bool> {
        self.airsim_client
            .unary_rpc(
                "setAngleLevelControllerGains".into(),
                Some(angle_level_gains.as_msgpack(self.vehicle_name)),
            )
            .await
            .map(|response| response.result.is_ok())
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude and altitude
    ///
    /// args:
    ///     rotation (Orientation3): Roll angle, pitch angle, and yaw angle set points are given in `radians`, in the ENU body frame.
    ///     z (f32): altitude z is given in local NED frame of the vehicle.
    ///     duration (f32): Desired amount of time (seconds), to send this command for
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
                    msgpack_rpc::Value::F32(rotation.roll),
                    msgpack_rpc::Value::F32(-rotation.pitch),
                    msgpack_rpc::Value::F32(-rotation.yaw),
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude and throttle in z-direction
    ///
    /// args:
    ///     rotation (Orientation3): Roll angle, pitch angle, and yaw angle set points are given in `radians`, in the ENU body frame.
    ///     throttle_z (f32): Desired throttle (between 0.0 to 1.0) in Z
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    pub async fn move_by_roll_pitch_yaw_throttle_async(
        &self,
        rotation: Orientation3,
        throttle_z: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        if throttle_z.is_sign_negative() || throttle_z > 1.0 {
            panic!("throttle_z outside of valid range 0.0 to 1.0")
        }

        self.airsim_client
            .unary_rpc(
                "moveByRollPitchYawThrottle".into(),
                Some(vec![
                    msgpack_rpc::Value::F32(rotation.roll),
                    msgpack_rpc::Value::F32(-rotation.pitch),
                    msgpack_rpc::Value::F32(-rotation.yaw),
                    msgpack_rpc::Value::F32(throttle_z),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude, yaw rate and throttle in z-direction
    ///
    /// args:
    ///     rotation (Orientation2): Desired roll and pitch angle set points are given in `radians`, in the ENU body frame.
    ///     yaw_rate (f32): Desired yaw rate, in radian per second.
    ///     throttle_z (f32): Desired throttle (between 0.0 to 1.0) in Z
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    pub async fn move_by_roll_pitch_yawrate_throttle_async(
        &self,
        rotation: Orientation2,
        yaw_rate: f32,
        throttle_z: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();
        if throttle_z.is_sign_negative() || throttle_z > 1.0 {
            panic!("throttle_z outside of valid range 0.0 to 1.0")
        }

        self.airsim_client
            .unary_rpc(
                "moveByRollPitchYawrateThrottle".into(),
                Some(vec![
                    msgpack_rpc::Value::F32(rotation.roll),
                    msgpack_rpc::Value::F32(-rotation.pitch),
                    msgpack_rpc::Value::F32(-yaw_rate),
                    msgpack_rpc::Value::F32(throttle_z),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude, yaw rate and altitude Z (absolute, not relative)
    ///
    /// args:
    ///     rotation (Orientation2): Desired roll and pitch angle set points are given in `radians`, in the ENU body frame.
    ///     yaw_rate (f32): Desired yaw rate, in radian per second.
    ///     z (f32): altitude z is given in local NED frame of the vehicle.
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    pub async fn move_by_roll_pitch_yawrate_z_async(
        &self,
        rotation: Orientation2,
        yaw_rate: f32,
        z: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByRollPitchYawrateZ".into(),
                Some(vec![
                    msgpack_rpc::Value::F32(rotation.roll),
                    msgpack_rpc::Value::F32(-rotation.pitch),
                    msgpack_rpc::Value::F32(-yaw_rate),
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude, yaw rate and altitude Z (absolute, not relative)
    ///
    /// args:
    ///     rotation_rates (Orientation2): Roll rate, pitch rate, and yaw rate set points are given in `radians`, in the body frame.
    ///     yaw_rate (f32): Desired yaw rate, in radian per second.
    ///     z (f32): altitude z is given in local NED frame of the vehicle.
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    pub async fn move_by_angle_rates_z_async(
        &self,
        rotation_rates: Orientation3,
        z: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "moveByAngleRatesZ".into(),
                Some(vec![
                    msgpack_rpc::Value::F32(rotation_rates.roll),
                    msgpack_rpc::Value::F32(-rotation_rates.pitch),
                    msgpack_rpc::Value::F32(-rotation_rates.yaw),
                    msgpack_rpc::Value::F32(z),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Low level control API
    ///
    /// Set an desired (absolute, not relative) attitude, yaw rate and altitude Z (absolute, not relative)
    ///
    /// args:
    ///     rotation_rates (Orientation2): Roll rate, pitch rate, and yaw rate set points are given in `radians`, in the body frame.
    ///     yaw_rate (f32): Desired yaw rate, in radian per second.
    ///     throttle (f32): Desired throttle (between 0.0 to 1.0)
    ///     duration (f32): Desired amount of time (seconds), to send this command for
    pub async fn move_by_angle_rates_throttle_async(
        &self,
        rotation_rates: Orientation3,
        throttle: f32,
        duration: f32,
    ) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();
        if throttle.is_sign_negative() || throttle > 1.0 {
            panic!("throttle outside of valid range 0.0 to 1.0")
        }

        self.airsim_client
            .unary_rpc(
                "moveByAngleRatesThrottle".into(),
                Some(vec![
                    msgpack_rpc::Value::F32(rotation_rates.roll),
                    msgpack_rpc::Value::F32(-rotation_rates.pitch),
                    msgpack_rpc::Value::F32(-rotation_rates.yaw),
                    msgpack_rpc::Value::F32(throttle),
                    msgpack_rpc::Value::F32(duration),
                    Value::String(vehicle_name),
                ]),
            )
            .await
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Get the kinematic state of the multirotor vehicle
    pub async fn get_multirotor_state(&self) -> NetworkResult<MultiRotorState> {
        let vehicle_name: Utf8String = self.vehicle_name.into();
        self.airsim_client
            .unary_rpc("getMultirotorState".into(), Some(vec![Value::String(vehicle_name)]))
            .await
            .map(MultiRotorState::from)
    }

    /// Used to obtain the current state of all a multirotor's rotors. The state includes the speeds,
    /// thrusts and torques for all rotors.
    pub async fn get_rotor_states(&self) -> NetworkResult<RotorStates> {
        let vehicle_name: Utf8String = self.vehicle_name.into();
        self.airsim_client
            .unary_rpc("getRotorStates".into(), Some(vec![Value::String(vehicle_name)]))
            .await
            .map(RotorStates::from)
    }

    /// Camera API
    ///
    /// Returns binary string literal of compressed png image in presented as an vector of bytes
    ///
    /// Returns bytes of png format image which can be dumped into abinary file to create .png image
    /// See https://microsoft.github.io/AirSim/image_apis/ for details
    ///
    /// args:
    ///     vehicle_name (Option<&str>): Name of the vehicle to send this command to
    ///     camera_name (String): Name of the camera, for backwards compatibility, ID numbers such as 0,1,etc. can also be used
    ///     image_type (ImageType): Type of image required
    ///     external (Option<bool>): Whether the camera is an External Camera
    #[inline(always)]
    pub async fn sim_get_image(
        &self,
        camera_name: &str,
        image_type: ImageType,
        external: Option<bool>,
    ) -> Result<CompressedImage, NetworkError> {
        self.airsim_client
            .sim_get_image(Some(self.vehicle_name), camera_name, image_type, external)
            .await
    }

    /// Camera API
    ///
    /// Get multiple images
    /// See https://microsoft.github.io/AirSim/image_apis/ for details and examples
    ///
    /// Args:
    ///     requests (ImageRequests): Images required
    ///     vehicle_name (Option<&str>): Name of vehicle associated with the camera
    ///     external (Option<bool>): Whether the camera is an External Camera
    #[inline(always)]
    pub async fn sim_get_images(&self, _requests: ImageRequests, _external: Option<bool>) -> Result<(), NetworkError> {
        // self.airsim_client
        //     .sim_get_images(requests, Some(self.vehicle_name), external)
        //     .await
        unimplemented!("todo");
    }
}
