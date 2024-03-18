use node_handshake::handshake::{start_handshakes, FIVE_SECONDS, PORT_BITCOIN};

// Test ~120 IP addresses
const TEST_NODES: [&str; 9] = [
    "mx.jamestrev.com",              // 1 IP address
    "mail.saxrag.com",               // 1 IP address
    "showy-toys.aeza.network",       // DNS name does not resolve
    "nickjlock.plus.com",            // 1 IP address
    "seed.btc.petertodd.org",        // 24 IP addresses
    "seed.bitcoin.sipa.be",          // 25 IP addresses
    "dnsseed.bluematt.me",           // 21 IP addresses
    "seed.bitcoinstats.com",         // 25 IP addresses
    "seed.bitcoin.jonasschnelli.ch", // 24 IP addresses
];

#[tokio::test]
async fn tests_multiple_dns_seed_nodes() {
    tracing_subscriber::fmt::init();

    for node in TEST_NODES {
        let _ = start_handshakes(node.to_owned(), PORT_BITCOIN, FIVE_SECONDS).await;
    }
}
