use rmp_rpc::{message::Response, Value, Utf8String};

#[derive(Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn to_msgpack(&self) -> Value {
        let x_val: Utf8String = "x_val".into();
        let y_val: Utf8String = "y_val".into();
        let z_val: Utf8String = "z_val".into();

        let val = Value::Map(vec![
            (Value::String(x_val), Value::F32(self.x)),
            (Value::String(y_val), Value::F32(self.y)),
            (Value::String(z_val), Value::F32(self.z))
        ]);
        
        let msg: Vec<(rmp_rpc::Value, rmp_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        let req = Value::Map(msg);
        req
    }
}
