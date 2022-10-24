use rmp_rpc::{message::Response, Value};

#[derive(Debug, Clone)]
/// List containing all the names of objects in the simulation scene
pub struct SceneObjects(pub Vec<String>);

impl From<Response> for SceneObjects {
    fn from(msgpack: Response) -> Self {
        let mut objects = vec![];

        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<Value> = res.as_array().unwrap();
                for s in payload {
                    let s = s.as_str().unwrap().to_string();
                    objects.push(s);
                }
            }
            Err(_) => panic!("Could not decode result from SceneObjects msgpack"),
        };

        SceneObjects(objects)
    }
}
