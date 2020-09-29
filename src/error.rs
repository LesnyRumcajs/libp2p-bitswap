use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitswapError {
    #[error("Error while reading from socket: {0}")]
    ReadError(#[from] libp2p::core::upgrade::ReadOneError),
    #[error("Error while decoding bitswap message: {0}")]
    ProtobufError(#[from] prost::DecodeError),
    #[error("Error while parsing cid: {0}")]
    Cid(#[from] tiny_cid::Error),
    #[error("Invalid message")]
    InvalidMessage,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
