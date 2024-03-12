use crate::handshake::PORT_BITCOIN;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct SeedArgs {
    pub dns_seed: String,

    #[arg(default_value_t = PORT_BITCOIN)]
    pub port: u16,
}
