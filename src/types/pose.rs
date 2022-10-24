use rmp_rpc::{message::Response, Value};

use crate::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Position3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position3 { x, y, z }
    }
}

impl From<Value> for Position3 {
    fn from(msgpack: Value) -> Self {
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();

        // position
        let mut points = vec![];
        println!("pos payload {payload:?}");
        for (_, v) in payload {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        Position3::new(points[0], points[1], points[2])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Orientation3 {
    /// roll angle, in radians
    pub roll: f32,
    /// pitch angle, in radians
    pub pitch: f32,
    /// yaw angle, in radians
    pub yaw: f32,
}

impl Orientation3 {
    pub fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Orientation3 { roll, pitch, yaw }
    }
}

impl From<Value> for Quaternion {
    fn from(msgpack: Value) -> Self {
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();

        // quaternion
        let mut quats = vec![];
        for (_, q_i) in payload {
            let q = q_i.as_f64().unwrap() as f32;
            quats.push(q);
        }
        Quaternion::new(quats[0], quats[1], quats[2], quats[3])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pose3 {
    pub position: Position3,
    pub orientation: Quaternion,
}

impl Pose3 {
    pub fn new(position: Position3, orientation: Quaternion) -> Self {
        Self { position, orientation }
    }
}

impl From<Response> for Pose3 {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();

                // position
                let position: Position3 = payload[0].1.to_owned().into();

                // orientation
                let orientation: Quaternion = payload[1].1.to_owned().into();

                Self { position, orientation }
            }
            Err(_) => panic!("Could not decode result from Pose3 msgpack"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Orientation2 {
    /// roll angle, in radians
    pub roll: f32,
    /// pitch angle, in radians
    pub pitch: f32,
}

impl Orientation2 {
    pub fn new(roll: f32, pitch: f32) -> Self {
        Orientation2 { roll, pitch }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity3 {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Velocity3 {
    pub fn new(vx: f32, vy: f32, vz: f32) -> Self {
        Velocity3 { vx, vy, vz }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity2 {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity2 {
    pub fn new(vx: f32, vy: f32) -> Self {
        Velocity2 { vx, vy }
    }
}

/// The kinematic state of the vehicle
#[derive(Debug, Clone, Copy)]
pub struct KinematicsState {
    /// position in the frame of the vehicle's starting point
    pub position: Position3,
    /// orientation in the frame of the vehicle's starting point
    pub orientation: Orientation3,
    /// linear velocity in ENU body frame
    pub linear_velocity: Vector3,
    /// angular velocity in ENU body frame
    pub angular_velocity: Vector3,
    /// linear acceleration in ENU body frame
    pub linear_acceleration: Vector3,
    /// angular acceleration in ENU body frame
    pub angular_acceleration: Vector3,
}

impl KinematicsState {
    pub fn new(
        position: Position3,
        orientation: Orientation3,
        linear_velocity: Vector3,
        angular_velocity: Vector3,
        linear_acceleration: Vector3,
        angular_acceleration: Vector3,
    ) -> Self {
        KinematicsState {
            position,
            orientation,
            linear_velocity,
            angular_velocity,
            linear_acceleration,
            angular_acceleration,
        }
    }
}

impl From<Value> for KinematicsState {
    fn from(msgpack: Value) -> Self {
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();

        // position
        let mut points = vec![];
        let position_msgpack: &Vec<(Value, Value)> = payload[0].1.as_map().unwrap();
        for (_, v) in position_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let position = Position3::new(points[0], points[1], points[2]);

        // orientation
        let mut points = vec![];
        let orientation_msgpack: &Vec<(Value, Value)> = payload[1].1.as_map().unwrap();
        for (_, v) in orientation_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let orientation = Orientation3::new(points[0], points[1], points[2]);

        // linear velocity
        let mut points = vec![];
        let linear_velocity_msgpack: &Vec<(Value, Value)> = payload[2].1.as_map().unwrap();
        for (_, v) in linear_velocity_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let linear_velocity = Vector3::new(points[0], points[1], points[2]);

        // angular velocity
        let mut points = vec![];
        let angular_velocity_msgpack: &Vec<(Value, Value)> = payload[3].1.as_map().unwrap();
        for (_, v) in angular_velocity_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let angular_velocity = Vector3::new(points[0], points[1], points[2]);

        // linear acceleration
        let mut points = vec![];
        let linear_acceleration_msgpack: &Vec<(Value, Value)> = payload[4].1.as_map().unwrap();
        for (_, v) in linear_acceleration_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let linear_acceleration = Vector3::new(points[0], points[1], points[2]);

        // linear acceleration
        let mut points = vec![];
        let angular_acceleration_msgpack: &Vec<(Value, Value)> = payload[5].1.as_map().unwrap();
        for (_, v) in angular_acceleration_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let angular_acceleration = Vector3::new(points[0], points[1], points[2]);

        Self {
            position,
            orientation,
            linear_velocity,
            angular_velocity,
            linear_acceleration,
            angular_acceleration,
        }
    }
}
