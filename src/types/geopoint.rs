use rmp_rpc::{message::Response, Value};

#[derive(Debug, Clone, Copy)]
pub struct GeoPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl GeoPoint {
    pub fn new(latitude: f32, longitude: f32, altitude: f32) -> Self {
        GeoPoint {
            latitude,
            longitude,
            altitude,
        }
    }
}

impl From<Response> for GeoPoint {
    fn from(msgpack: Response) -> Self {
        let mut points = vec![];

        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                for (_, v) in payload {
                    let p = v.as_f64().unwrap() as f32;
                    points.push(p);
                }
            }
            Err(_) => panic!("Could not decode result from GeoPoint msgpack"),
        };

        GeoPoint {
            latitude: points[0],
            longitude: points[1],
            altitude: points[2],
        }
    }
}

impl From<Value> for GeoPoint {
    fn from(msgpack: Value) -> Self {
        let mut points = vec![];
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();
        for (_, v) in payload {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }

        GeoPoint {
            latitude: points[0],
            longitude: points[1],
            altitude: points[2],
        }
    }
}
