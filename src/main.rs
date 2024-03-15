use clap::Parser;
use node_handshake::{
    error::Result,
    handshake::{start_handshakes, FIVE_SECONDS, PORT_BITCOIN},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Parser)]
pub struct SeedArgs {
    pub dns_seed_name: String,

    #[arg(default_value_t = PORT_BITCOIN)]
    pub port: u16,

    #[arg(default_value_t = FIVE_SECONDS.as_secs())]
    pub timeout: u64,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = SeedArgs::parse();

    start_handshakes(args.dns_seed_name, args.port, args.timeout).await
}
