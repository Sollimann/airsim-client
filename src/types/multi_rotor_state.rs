use rmp_rpc::{message::Response, Value};

use crate::GeoPoint;

use super::{collision_info::CollisionInfo, pose::KinematicsState, rc_data::RCDataState};

#[derive(Debug, Clone, Copy)]
pub enum LandedState {
    Landed, // 0
    Flying, // 1
}

impl From<Value> for LandedState {
    fn from(msgpack: Value) -> Self {
        let landed = msgpack.as_u64().unwrap();
        if landed == 0 {
            LandedState::Landed
        } else if landed == 1 {
            LandedState::Flying
        } else {
            panic!("could not convert value {landed:?}. Has to be either Landed(0) or Flying(1)");
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiRotorState {
    pub collision: CollisionInfo,
    pub kinematics_estimated: KinematicsState,
    pub gps_location: GeoPoint,
    pub timestamp: u64,
    pub landed_state: LandedState,
    pub rc_data: RCDataState,
}

impl From<Response> for MultiRotorState {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();

                // collision
                let collision: CollisionInfo = payload[0].1.to_owned().into();

                // kinematics estimated
                let kinematics_estimated: KinematicsState = payload[1].1.to_owned().into();

                // gps location
                let gps_location: GeoPoint = payload[2].1.to_owned().into();

                // timestamp
                let timestamp = payload[3].1.as_u64().unwrap();

                // landed state
                let landed_state: LandedState = payload[4].1.to_owned().into();

                // rc data
                let rc_data: RCDataState = payload[5].1.to_owned().into();

                Self {
                    collision,
                    kinematics_estimated,
                    gps_location,
                    timestamp,
                    landed_state,
                    rc_data,
                }
            }
            Err(_) => panic!("Could not decode result from MultiRotorState msgpack"),
        }
    }
}
