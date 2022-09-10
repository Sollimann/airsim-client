use rmp_rpc::{message::Response, Value};

#[derive(Debug)]
pub struct GeoPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
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
