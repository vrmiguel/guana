use argh::FromArgs;

#[derive(FromArgs)]
/// A fast gRPC server to filter User-Agent strings.
pub struct Args {
    /// the address in which the service will be served
    #[argh(option)]
    pub address: Option<String>,
}
