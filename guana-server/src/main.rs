mod cli;

use std::net::SocketAddr;

use guana_service::{proto, user_agent_analyzer_service};
use tonic_reflection::server::{
    ServerReflection, ServerReflectionServer,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("Can't serve on {0} since it's an invalid address")]
    InvalidAddress(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Parse user-supplied CLI args and get the server address from it
fn address_from_arg() -> Result<SocketAddr> {
    let cli::Args { address } = argh::from_env();

    let address = address.as_deref().unwrap_or_else(|| {
        tracing::info!(
            "Server address not set. Using a default value"
        );

        "[::1]:50051"
    });

    address
        .parse()
        .map_err(|_| Error::InvalidAddress(address.into()))
}

#[tokio::main]
async fn main() -> Result {
    // Start tracing
    tracing_subscriber::fmt().compact().init();

    let addr = address_from_arg()?;
    tracing::info!("Starting server on {addr}");

    tonic::transport::Server::builder()
        .trace_fn(|_| tracing::info_span!("user_agent_server"))
        .add_service(reflection_service())
        .add_service(user_agent_analyzer_service())
        .serve(addr)
        .await?;

    Ok(())
}

// Allows the gRPC server to answer the following queries:
//
// 1. What methods does a server export?
// 2. For a particular method, how do we call it?
//    Specifically, what are the names of the methods, are those methods unary or streaming,
//    and what are the types of the argument and result?
fn reflection_service(
) -> ServerReflectionServer<impl ServerReflection> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            proto::FILE_DESCRIPTOR_SET,
        )
        .build()
        .expect("invalid file descriptor set")
}
