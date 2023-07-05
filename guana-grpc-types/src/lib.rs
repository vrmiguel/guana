pub mod proto {
    // Include the generated Protobuf server and client items
    // defined in `user_agent_analyzer.proto`.
    tonic::include_proto!("user_agent_analyzer");

    /// Saves a binary representation of the protocol buffer definitions for the service, message, and enum used in our gRPC service.
    ///
    /// This allows clients and servers to exchange protocol buffer definitions at runtime.
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!(
        "user_agent_analyzer_descriptor"
    );
}
