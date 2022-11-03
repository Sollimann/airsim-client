use msgpack_rpc::{message::Response, Value};

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Scene,
    DepthPlanar,
    DepthPerspective,
    DepthVis,
    DisparityNormalized,
    SurfaceNormals,
    Infrared,
    OpticalFlow,
    OpticalFlowVis,
}

impl ImageType {
    pub(crate) fn as_msgpack(&self) -> Value {
        let val = match self {
            ImageType::Scene => 0_i64,
            ImageType::DepthPlanar => 1_i64,
            ImageType::DepthPerspective => 2_i64,
            ImageType::DepthVis => 3_i64,
            ImageType::DisparityNormalized => 4_i64,
            ImageType::SurfaceNormals => 5_i64,
            ImageType::Infrared => 6_i64,
            ImageType::OpticalFlow => 7_i64,
            ImageType::OpticalFlowVis => 8_i64,
        };

        Value::Integer(val.into())
    }
}

#[derive(Debug, Clone)]
/// Binary string literal of compressed png image in presented as an vector of bytes
pub struct CompressedImage(pub Vec<u8>);

impl From<Response> for CompressedImage {
    fn from(msgpack: Response) -> Self {
        let mut pixels = vec![];

        match msgpack.result {
            Ok(res) => {
                println!("image: {res:?}");
                let payload: &Vec<Value> = res.as_array().unwrap();
                for v in payload {
                    let p = v.as_u64().unwrap() as u8;
                    pixels.push(p);
                }
            }
            Err(_) => panic!("Could not decode result from CompressedImage msgpack"),
        };

        Self(pixels)
    }
}
