#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    // Note: This will never be `tonic::Status::Ok` in here
    #[error("Got a gRPC status {0}")]
    GrpcStatus(#[from] tonic::Status),
}

pub type Result<T = ()> = std::result::Result<T, Error>;
