use clap::Parser;

const DEFAULT_PORT: u16 = 8333;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct SeedArgs {
    pub dns_seed: String,

    #[arg(default_value_t = DEFAULT_PORT)]
    pub port: u16,
}
