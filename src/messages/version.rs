use bitcoin::{
    p2p::{self, address, message, message_network},
    secp256k1::rand::{self, Rng},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::messages::PROTOCOL_VERSION;

// Message format as per documentation https://en.bitcoin.it/wiki/Protocol_documentation#version
pub fn version_msg(to_address: SocketAddr) -> message::NetworkMessage {
    // Various message fields do not need to be populated, either because they will be ignored anyway, or because this is
    // just a is minimal PoC implementation
    //
    // The new() method belonging to message_network::VersionMessage is not called because this hard codes the message
    // version to 70001
    let ignore_from_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("error: Unable to get system time")
        .as_secs();

    message::NetworkMessage::Version(message_network::VersionMessage {
        version: PROTOCOL_VERSION,
        services: p2p::ServiceFlags::NONE,
        timestamp: now as i64,
        receiver: address::Address::new(&to_address, p2p::ServiceFlags::NONE),
        sender: address::Address::new(&ignore_from_address, p2p::ServiceFlags::NONE),
        nonce: rand::thread_rng().gen(),
        user_agent: "P2P Handshake PoC".to_owned(),
        start_height: 0,
        relay: false,
    })
}
