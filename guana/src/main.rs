mod cli;

use std::net::SocketAddr;

use guana_server::user_agent_analyzer_router;

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
    user_agent_analyzer_router().serve(addr).await?;

    Ok(())
}
