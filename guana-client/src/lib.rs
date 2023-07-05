mod error;

pub use error::Result;
pub use guana_grpc_types::proto::analyze_response::Decision;

use guana_grpc_types::proto::{
    self, user_agent_analyzer_client::UserAgentAnalyzerClient,
};
use tonic::transport::Channel;

pub struct GuanaClient {
    inner: UserAgentAnalyzerClient<Channel>,
}

impl GuanaClient {
    /// Attempts to connect to the server at the given address.
    pub async fn connect(
        addr: impl Into<String>,
    ) -> Result<Self> {
        let inner =
            UserAgentAnalyzerClient::connect(addr.into())
                .await?;

        Ok(Self { inner })
    }

    /// Sends the given User-Agent string to the gRPC server
    /// and returns it's decision, i.e. whether to accept it or not.
    pub async fn check_user_agent(
        &mut self,
        user_agent: impl Into<String>,
    ) -> Result<Decision> {
        let req = proto::UserAgentRequest {
            user_agent: user_agent.into(),
        };

        let response = self
            .inner
            .analyze_user_agent(req)
            .await?
            .into_inner();

        Ok(response.decision())
    }
}
