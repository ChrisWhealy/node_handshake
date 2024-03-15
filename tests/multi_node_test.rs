use node_handshake::handshake::{start_handshakes, FIVE_SECONDS, PORT_BITCOIN};

// Test about 100 IP addresses
const TEST_NODES: [&str; 9] = [
    "mx.jamestrev.com",
    "mail.saxrag.com",
    "showy-toys.aeza.network",
    "nickjlock.plus.com",
    "seed.btc.petertodd.org",
    "seed.bitcoin.sipa.be",
    "dnsseed.bluematt.me",
    "seed.bitcoinstats.com",
    "seed.bitcoin.jonasschnelli.ch",
];

#[tokio::test]
async fn tests_multiple_dns_seed_nodes() {
    tracing_subscriber::fmt::init();

    for node in TEST_NODES {
        let _ = start_handshakes(node.to_owned(), PORT_BITCOIN, FIVE_SECONDS.as_secs()).await;
    }
}
