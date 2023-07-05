use std::ops::Not;

use guana_grpc_types::proto::{
    self, user_agent_analyzer_server::UserAgentAnalyzer,
};

use guana_grpc_types::proto::analyze_response::Decision;

use tonic::{Request, Response};

use crate::TonicResult;

/// Given an `User-Agent`
fn handle_user_agent(user_agent: &str) -> Decision {
    fn is_safari(user_agent: &str) -> bool {
        let is_safari = user_agent.contains("Safari");
        let is_not_chrome = user_agent.contains("Chrome").not();

        // Distinguish between WebKit browsers
        is_safari && is_not_chrome
    }

    if user_agent.contains("Firefox") {
        Decision::Allow
    } else if is_safari(user_agent) {
        Decision::Block
    } else {
        Decision::Unknown
    }
}

/// Concrete type that implements the `UserAgentAnalyzer` service defined in the Protobuf schema.
#[derive(Default)]
pub struct UserAgentAnalyzerImpl {}

#[tonic::async_trait]
impl UserAgentAnalyzer for UserAgentAnalyzerImpl {
    async fn analyze_user_agent(
        &self,
        request: Request<proto::UserAgentRequest>,
    ) -> TonicResult<proto::AnalyzeResponse> {
        let mut reply = proto::AnalyzeResponse::default();

        // If we know the caller's address, log it
        if let Some(addr) = request.remote_addr() {
            tracing::info!("Received a request from {addr}",);
        } else {
            tracing::info!(
                "Received a request from an unnamed client"
            );
        };

        // Check the User-Agent and decide on whether or not to accept it
        {
            let user_agent = &request.into_inner().user_agent;
            let decision = handle_user_agent(user_agent);

            reply.set_decision(decision);
        };

        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod decision_tests {
    use std::assert_eq;

    use guana_grpc_types::proto::analyze_response::Decision;

    use crate::user_agent_analyzer::handle_user_agent;

    #[test]
    fn allows_firefox_user_agents() {
        assert_eq!(
            handle_user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0"),
            Decision::Allow
        );
        assert_eq!(
            handle_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0"),
            Decision::Allow
        );
        assert_eq!(
            handle_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:90.0) Gecko/20100101 Firefox/90.0"),
            Decision::Allow
        )
    }

    #[test]
    fn denies_safari_user_agents() {
        assert_eq!(
            handle_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15"),
            Decision::Block
        );
    }

    #[test]
    fn returns_unknown_for_other_user_agents() {
        assert_eq!(
            handle_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 Edg/92.0.902.67    "),
            Decision::Unknown
        );
    }
}
