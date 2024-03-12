pub mod dns_name_resolver;
pub mod handshake;
pub mod messages;
pub mod parse_args;

pub const DEFAULT_SEED_NODES: &[&str] = &[
    "seed.bitcoin.sipa.be.",
    "dnsseed.bluematt.me.",
    "seed.bitcoinstats.com.",
    "seed.bitcoin.jonasschnelli.ch.",
    "seed.btc.petertodd.org.",
];
