use msgpack_rpc::{Utf8String, Value};

#[derive(Debug, Clone, Copy)]
pub struct YawMode {
    is_rate: bool,
    yaw_or_rate: f32,
}

impl YawMode {
    pub fn new(is_rate: bool, yaw_or_rate: f32) -> Self {
        Self { is_rate, yaw_or_rate }
    }

    pub fn as_msgpack(&self) -> Value {
        let is_rate_str: Utf8String = "is_rate".into();
        let yaw_or_rate_str: Utf8String = "yaw_or_rate".into();

        let val = Value::Map(vec![
            (Value::String(is_rate_str), Value::Boolean(self.is_rate)),
            (Value::String(yaw_or_rate_str), Value::F32(self.yaw_or_rate)),
        ]);
        let msg: Vec<(msgpack_rpc::Value, msgpack_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}
