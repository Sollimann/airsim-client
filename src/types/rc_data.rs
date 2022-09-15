use rmp_rpc::{Utf8String, Value};

use super::pose::Orientation3;

#[derive(Clone, Debug)]
pub struct RCData {
    pub timestamp: u64,
    pub orientation: Orientation3,
    pub throttle: f32,
    pub switch: [u32; 8],
    pub is_initialized: bool,
    pub is_valid: bool,
}

impl RCData {
    pub fn new(
        timestamp: u64,
        orientation: Orientation3,
        throttle: f32,
        switch: Option<[u32; 8]>,
        is_initialized: bool,
        is_valid: bool,
    ) -> Self {
        let switch = switch.unwrap_or([0; 8]);

        Self {
            timestamp,
            orientation,
            throttle,
            switch,
            is_initialized,
            is_valid,
        }
    }

    pub(crate) fn to_msgpack(&self) -> Value {
        let timestamp: Utf8String = "timestamp".into();
        let pitch: Utf8String = "pitch".into();
        let roll: Utf8String = "roll".into();
        let throttle: Utf8String = "throttle".into();
        let yaw: Utf8String = "yaw".into();
        let switch1: Utf8String = "switch1".into();
        let switch2: Utf8String = "switch2".into();
        let switch3: Utf8String = "switch3".into();
        let switch4: Utf8String = "switch4".into();
        let switch5: Utf8String = "switch5".into();
        let switch6: Utf8String = "switch6".into();
        let switch7: Utf8String = "switch7".into();
        let switch8: Utf8String = "switch8".into();
        let is_initialized: Utf8String = "is_initialized".into();
        let is_valid: Utf8String = "is_valid".into();

        let val = Value::Map(vec![
            (Value::String(timestamp), Value::Integer(self.timestamp.into())),
            (Value::String(pitch), Value::F32(self.orientation.pitch)),
            (Value::String(roll), Value::F32(self.orientation.roll)),
            (Value::String(throttle), Value::F32(self.throttle)),
            (Value::String(yaw), Value::F32(self.orientation.yaw)),
            (Value::String(switch1), Value::Integer(self.switch[0].into())),
            (Value::String(switch2), Value::Integer(self.switch[1].into())),
            (Value::String(switch3), Value::Integer(self.switch[2].into())),
            (Value::String(switch4), Value::Integer(self.switch[3].into())),
            (Value::String(switch5), Value::Integer(self.switch[4].into())),
            (Value::String(switch6), Value::Integer(self.switch[5].into())),
            (Value::String(switch7), Value::Integer(self.switch[6].into())),
            (Value::String(switch8), Value::Integer(self.switch[7].into())),
            (Value::String(is_initialized), Value::Boolean(self.is_initialized)),
            (Value::String(is_valid), Value::Boolean(self.is_valid)),
        ]);

        let msg: Vec<(rmp_rpc::Value, rmp_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}
