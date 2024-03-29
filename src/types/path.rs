use msgpack_rpc::Value;

use crate::Vector3;

#[derive(Debug, Clone)]
pub struct Path(pub Vec<Vector3>);

impl Path {
    pub(crate) fn as_msgpack(&self) -> Value {
        let v3_msgpack = self.0.iter().cloned().map(|v3| v3.as_msgpack()).collect();
        Value::Array(v3_msgpack)
    }
}
