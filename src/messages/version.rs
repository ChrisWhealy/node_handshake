use bitcoin::{
    p2p::{self, address, message, message_network},
    secp256k1::rand::{self, Rng},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};

// Message format as per documentation https://en.bitcoin.it/wiki/Protocol_documentation#version
pub fn version_msg(to_address: SocketAddr) -> message::NetworkMessage {
    // Various message fields do not need to be populated, either because they will be ignored anyway, or because this is
    // just a is minimal PoC implementation
    let ignore_from_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("error: Unable to get system time")
        .as_secs();

    // message_network::VersionMessage::new() hard codes the message version to 70001
    message::NetworkMessage::Version(message_network::VersionMessage::new(
        Default::default(),
        now as i64,
        address::Address::new(&to_address, p2p::ServiceFlags::NONE),
        address::Address::new(&ignore_from_address, p2p::ServiceFlags::NONE),
        rand::thread_rng().gen(),
        "P2P Handshake PoC".to_owned(),
        0)
    )
}
