mod messages;
mod parse_args;

pub mod dns_name_resolver;
pub mod error;
pub mod handshake;

pub use self::dns_name_resolver::*;
pub use self::error::*;
pub use self::handshake::*;
pub use self::messages::*;
pub use self::parse_args::*;

pub const DEFAULT_SEED_NODES: &[&str] = &[
    "seed.btc.petertodd.org.",
    "seed.bitcoin.sipa.be.",
    "dnsseed.bluematt.me.",
    "seed.bitcoinstats.com.",
    "seed.bitcoin.jonasschnelli.ch.",
];
