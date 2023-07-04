pub mod proto {
    // Include the generated Protobuf server and client items
    // defined in `user_agent_analyzer.proto`.
    tonic::include_proto!("user_agent_analyzer");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!(
        "user_agent_analyzer_descriptor"
    );
}
