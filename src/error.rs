use async_std::channel::RecvError;
use msgpack_rpc::DecodeError;
use std::io;
use thiserror::Error;

pub type NetworkResult<T> = Result<T, NetworkError>;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("received because the channel is empty or closed")]
    Recv(#[from] RecvError),
    #[error("issue with read or write I/O operation")]
    Io(#[from] io::Error),
    #[error("Could not send message: {message}")]
    Send { message: String },
    #[error("Could not decode the message that was received")]
    Decode(#[from] DecodeError),
}
