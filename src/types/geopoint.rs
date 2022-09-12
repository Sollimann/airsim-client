use rmp_rpc::{message::Response, Value, Utf8String};

#[derive(Debug)]
pub struct GeoPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl GeoPoint {
    pub fn new(latitude: f32, longitude: f32, altitude: f32) -> Self {
        GeoPoint { latitude, longitude, altitude }
    }

    pub(crate) fn to_msgpack(&self) -> Value {
        let latitude: Utf8String = "latitude".into();
        let longitude: Utf8String = "longitude".into();
        let altitude: Utf8String = "altitude".into();

        let val = Value::Map(vec![
            (Value::String(latitude), Value::F32(self.latitude)),
            (Value::String(longitude), Value::F32(self.longitude)),
            (Value::String(altitude), Value::F32(self.altitude))
        ]);
        
        let msg: Vec<(rmp_rpc::Value, rmp_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        let req = Value::Map(msg);
        req
    }
}

impl From<Response> for GeoPoint {
    fn from(msgpack: Response) -> Self {
        let mut points = vec![];

        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                for (_, v) in payload {
                    let p = v.as_f64().unwrap();
                    points.push(p);
                }
            }
            Err(_) => panic!("Could not decode result from GeoPoint msgpack"),
        };

        GeoPoint {
            latitude: points[0] as f32,
            longitude: points[1] as f32,
            altitude: points[2] as f32,
        }
    }
}
