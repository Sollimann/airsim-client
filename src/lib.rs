pub use clients::airsim_client::AirsimClient;
pub use clients::car_client::CarClient;
pub use clients::multi_rotor_client::MultiRotorClient;
pub use error::{DecodeError, NetworkError, NetworkResult};
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
