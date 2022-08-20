pub use client::MultiRotorClient;
pub use error::{DecodeError, NetworkError, NetworkResult};
pub(crate) use msgpack::MsgPackClient;

mod client;
mod error;
mod msgpack;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
