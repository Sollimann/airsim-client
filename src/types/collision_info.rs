use rmp_rpc::Value;

use crate::Vector3;

#[derive(Clone, Debug)]
pub struct CollisionInfo {
    pub has_collided: bool,
    pub penetration_depth: f32,
    pub timestamp: u64,
    pub normal: Vector3,
    pub impact_point: Vector3,
    pub position: Vector3,
    pub object_name: String,
    pub object_id: i64,
}

impl From<Value> for CollisionInfo {
    fn from(msgpack: Value) -> Self {
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();

        // has collided
        let has_collided = payload[0].1.as_bool().unwrap();

        // penetration depth
        let penetration_depth = payload[1].1.as_f64().unwrap() as f32;

        // timestamp
        let timestamp = payload[2].1.as_u64().unwrap();

        // normal
        let mut points = vec![];
        let normal_msgpack: &Vec<(Value, Value)> = payload[3].1.as_map().unwrap();
        for (_, v) in normal_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let normal = Vector3::new(points[0], points[1], points[2]);

        // impact point
        let mut points = vec![];
        let impact_msgpack: &Vec<(Value, Value)> = payload[4].1.as_map().unwrap();
        for (_, v) in impact_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let impact_point = Vector3::new(points[0], points[1], points[2]);

        // position
        let mut points = vec![];
        let position_msgpack: &Vec<(Value, Value)> = payload[5].1.as_map().unwrap();
        for (_, v) in position_msgpack {
            let p = v.as_f64().unwrap() as f32;
            points.push(p);
        }
        let position = Vector3::new(points[0], points[1], points[2]);

        // object name
        let object_name = payload[6].1.as_str().unwrap().to_string();

        // object id
        let object_id = payload[7].1.as_i64().unwrap();

        Self {
            has_collided,
            penetration_depth,
            timestamp,
            normal,
            impact_point,
            position,
            object_name,
            object_id,
        }
    }
}
