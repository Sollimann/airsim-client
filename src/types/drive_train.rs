use rmp_rpc::Value;

#[derive(Debug)]
pub enum DrivetrainType {
    MaxDegreeOfFreedom,
    ForwardOnly,
}

impl DrivetrainType {
    pub fn to_msgpack(&self) -> Value {
        let val = match self {
            DrivetrainType::MaxDegreeOfFreedom => 1_i64,
            DrivetrainType::ForwardOnly => 2_i64,
        };

        Value::Integer(val.into())
    }
}
