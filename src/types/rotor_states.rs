use msgpack_rpc::{message::Response, Value};

#[derive(Debug, Clone, Copy)]
pub struct RotorState {
    pub thrust: f32,
    pub torque_scaler: f32,
    pub speed: f32,
}

impl From<Value> for RotorState {
    fn from(msgpack: Value) -> Self {
        let mut states = vec![];
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();
        for (_, v) in payload {
            let s = v.as_f64().unwrap() as f32;
            states.push(s);
        }

        RotorState {
            thrust: states[0],
            torque_scaler: states[1],
            speed: states[2],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RotorStates {
    pub rotors: [RotorState; 4],
    pub timestamp: u64,
}

impl From<Response> for RotorStates {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();

                // rotors
                let rotors: Vec<RotorState> = payload[0]
                    .1
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect();

                // timestamp
                let timestamp: u64 = payload[1].1.to_owned().as_u64().unwrap();

                RotorStates {
                    rotors: [rotors[0], rotors[1], rotors[2], rotors[3]],
                    timestamp,
                }
            }
            Err(_) => panic!("Could not decode result from RotorState(s) msgpack"),
        }
    }
}
