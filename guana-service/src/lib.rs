pub use guana_grpc_types::proto;
use guana_grpc_types::proto::user_agent_analyzer_server::UserAgentAnalyzerServer;
use tonic::{Response, Status};
use user_agent_analyzer::UserAgentAnalyzerImpl;

/// Module for the Rust-side implementation fo the `UserAgentAnalyzer` Proto service
mod user_agent_analyzer;

type TonicResult<T> = std::result::Result<Response<T>, Status>;

/// A gRPC transport server builder.
///
/// Adds the `UserAgentAnalyzer` service, alongside a reflection service.
///
/// ## Usage:
///
/// ```no_run
/// use guana_server::user_agent_analyzer_service;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let addr = "[::1]:50051".parse().unwrap();
///
///     user_agent_analyzer_router()
///         .serve(addr)
///         .await?;
///     tonic::transport::Server::builder()
///         .add_service(user_agent_analyzer_service())
///         .serve(addr)
///         .await?;
///
///     Ok(())
/// }
/// ```
pub fn user_agent_analyzer_service(
) -> UserAgentAnalyzerServer<UserAgentAnalyzerImpl> {
    UserAgentAnalyzerServer::new(UserAgentAnalyzerImpl::default())
}
