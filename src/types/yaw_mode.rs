use rmp_rpc::Value;

pub struct YawMode {
    is_rate: bool,
    yaw_or_rate: f32,
}

impl YawMode {
    pub fn new(is_rate: bool, yaw_or_rate: f32) -> Self {
        Self { is_rate, yaw_or_rate }
    }

    pub fn to_msgpack(&self) -> Value {
        let val = Value::Map(vec![(Value::Boolean(self.is_rate), Value::F32(self.yaw_or_rate))]);
        let msg: Vec<(rmp_rpc::Value, rmp_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}
