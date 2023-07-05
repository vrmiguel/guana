use guana_grpc_types::proto::{
    self, user_agent_analyzer_server::UserAgentAnalyzerServer,
};
use tonic::{transport::server::Router, Response, Status};
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
/// use guana_server::user_agent_analyzer_router;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let addr = "[::1]:50051".parse().unwrap();
///
///     user_agent_analyzer_router()
///         .serve(addr)
///         .await?;
///
///     Ok(())
/// }
/// ```
pub fn user_agent_analyzer_router() -> Router {
    // Allows the gRPC server to answer the following queries:
    //
    // 1. What methods does a server export?
    // 2. For a particular method, how do we call it?
    //    Specifically, what are the names of the methods, are those methods unary or streaming,
    //    and what are the types of the argument and result?
    let reflection_service =
        tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                proto::FILE_DESCRIPTOR_SET,
            )
            .build()
            .expect("invalid file descriptor set");

    let user_agent_service = UserAgentAnalyzerServer::new(
        UserAgentAnalyzerImpl::default(),
    );

    tonic::transport::Server::builder()
        .trace_fn(|_| tracing::info_span!("user_agent_server"))
        .add_service(reflection_service)
        .add_service(user_agent_service)
}
