use msgpack_rpc::{Utf8String, Value};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub(crate) fn as_msgpack(&self) -> Value {
        let x_val: Utf8String = "x_val".into();
        let y_val: Utf8String = "y_val".into();
        let z_val: Utf8String = "z_val".into();

        let val = Value::Map(vec![
            (Value::String(x_val), Value::F32(self.x)),
            (Value::String(y_val), Value::F32(self.y)),
            (Value::String(z_val), Value::F32(self.z)),
        ]);

        let msg: Vec<(msgpack_rpc::Value, msgpack_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}
