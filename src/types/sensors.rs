use msgpack_rpc::{message::Response, Utf8String, Value};

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
                let slice: &[u8] = res.as_slice().unwrap();
                for p in slice {
                    pixels.push(*p);
                }
            }
            Err(_) => panic!("Could not decode result from CompressedImage msgpack"),
        };

        Self(pixels)
    }
}

#[derive(Debug, Clone)]
pub struct ImageRequest {
    pub camera_name: String,
    pub image_type: ImageType,
    pub pixels_as_float: bool,
    pub compress: bool,
}

#[derive(Debug, Clone)]
pub struct ImageRequests(pub Vec<ImageRequest>);

impl ImageRequest {
    pub(crate) fn as_msgpack(&self) -> Value {
        let camera_name: Utf8String = "camera_name".into();
        let image_type: Utf8String = "image_type".into();
        let pixels_as_float: Utf8String = "pixels_as_float".into();
        let compress: Utf8String = "compress".into();

        let val = Value::Map(vec![
            (
                Value::String(camera_name),
                Value::String(self.camera_name.to_owned().into()),
            ),
            (Value::String(image_type), self.image_type.as_msgpack()),
            (Value::String(pixels_as_float), Value::Boolean(self.pixels_as_float)),
            (Value::String(compress), Value::Boolean(self.compress)),
        ]);

        let msg: Vec<(msgpack_rpc::Value, msgpack_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}

impl ImageRequests {
    pub(crate) fn as_msgpack(&self) -> Value {
        let images = self.0.iter().cloned().map(|img| img.as_msgpack()).collect();
        Value::Array(images)
    }
}
