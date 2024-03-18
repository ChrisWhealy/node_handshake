use std::fmt::Display;

use clap::Parser;
use node_handshake::{
    error::Result,
    handshake::{start_handshakes, FIVE_SECONDS, PORT_BITCOIN},
};
use tracing::info;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Parser)]
pub struct SeedArgs {
    pub dns_seed_name: String,

    #[arg(default_value_t = PORT_BITCOIN)]
    pub port: u16,

    #[arg(default_value_t = FIVE_SECONDS.as_millis() as u64)]
    pub timeout: u64,
}

impl Display for SeedArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Attempting handshakes with {}:{}  Timeout = {} ms",
            self.dns_seed_name, self.port, self.timeout
        )
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = SeedArgs::parse();
    info!("{}", args);

    start_handshakes(args.dns_seed_name, args.port, args.timeout).await
}
