pub use clients::airsim_client::AirsimClient;
pub use clients::car_client::CarClient;
pub use clients::multi_rotor_client::MultiRotorClient;
pub use error::{NetworkError, NetworkResult};
pub use msgpack_rpc::DecodeError;
pub use types::drive_train::DrivetrainType;
pub use types::gains::{AngularControllerGains, LinearControllerGains, PIDGains};
pub use types::geopoint::GeoPoint;
pub use types::image::{CompressedImage, ImageRequest, ImageRequests, ImageType};
pub use types::path::Path;
pub use types::pose::{Orientation2, Orientation3, Pose3, Position3, Quaternion, Velocity2, Velocity3};
pub use types::pwm::PWM;
pub use types::rc_data::RCData;
pub use types::rotor_states::{RotorState, RotorStates};
pub use types::simulation::SceneObjects;
pub use types::vector::Vector3;
pub use types::weather::WeatherParameter;
pub use types::yaw_mode::YawMode;

pub(crate) use msgpack::MsgPackClient;
mod clients;
mod error;
mod msgpack;
mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
