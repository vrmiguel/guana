<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana-service </h2>

[Tower](https://docs.rs/tower/latest/tower/) service that implements the [`UserAgentAnalyzer`](../proto/user_agent_analyzer.proto) gRPC service.

This crate is useful if you'd like to embed [`UserAgentAnalyzer`](../proto/user_agent_analyzer.proto) into a larger gRPC service

Example:

```rust
    use guana_service::user_agent_analyzer_service;

    tonic::transport::Server::builder()
        // Your options 
        .add_service(your_services)
        .add_service(user_agent_analyzer_service())
        .serve(your_address)
        .await?;
```

If you'd like to run this service standalone, take a look at [`guana-server`](../guana-server).