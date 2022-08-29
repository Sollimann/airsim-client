use rmp_rpc::Value;

#[derive(Debug)]
pub struct GeoPoint {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

impl From<Value> for GeoPoint {
    fn from(msgpack: Value) -> Self {
        match msgpack {
            Value::Nil => todo!(),
            Value::Boolean(_) => todo!(),
            Value::Integer(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Binary(_) => todo!(),
            Value::Array(_) => todo!(),
            Value::Map(_) => todo!(),
            Value::Ext(_, _) => todo!(),
        }
    }
}
