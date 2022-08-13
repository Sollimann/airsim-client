pub use msgpack::MsgPackClient;
pub use error::{NetworkError, DecodeError};

mod msgpack;
mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
