use clap::Parser;
use node_handshake::{
    error::Result,
    handshake::{start_handshakes, FIVE_SECONDS, PORT_BITCOIN},
};

// Test about 100 IP addresses
// const TEST_NODES: [&str; 7] = [
//     "mx.jamestrev.com",
//     "mail.saxrag.com",
//     "seed.btc.petertodd.org",
//     "seed.bitcoin.sipa.be",
//     "dnsseed.bluematt.me",
//     "seed.bitcoinstats.com",
//     "seed.bitcoin.jonasschnelli.ch",
// ];

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

    // for node in TEST_NODES {
    //     let _ = start_handshakes(node.to_owned(), args.port, args.timeout).await;
    // }

    // Ok(())

    start_handshakes(args.dns_seed_name, args.port, args.timeout).await
}
