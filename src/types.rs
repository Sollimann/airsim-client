use rmp_rpc::{Value, message::Response};


#[derive(Debug)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
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
            },
            Err(_) => panic!("Could not decode result from GeoPoint msgpack"),
        };

        GeoPoint { latitude: points[0], longitude: points[1], altitude: points[2] }
    }
}
