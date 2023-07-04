use guana_server::user_agent_analyzer_router;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result {
    // Start tracing
    tracing_subscriber::fmt().compact().init();

    let addr = "[::1]:50051".parse().unwrap();

    user_agent_analyzer_router().serve(addr).await?;

    Ok(())
}
