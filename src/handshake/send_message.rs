use bitcoin::{
    consensus::encode,
    p2p::{message, PROTOCOL_VERSION},
};
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};
use tracing::info;

use crate::{
    error::Result,
    messages::{verack::verack_msg, version::version_msg},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn send_msg_version(target_node: &SocketAddr, write_stream: &mut TcpStream) -> Result<()> {
    let payload = version_msg(*target_node);
    let msg = message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), payload);
    let msg_bytes = encode::serialize(&msg);

    info!(
        "version: Sending {} ({} bytes) to target node {}",
        PROTOCOL_VERSION,
        msg_bytes.len(),
        target_node
    );
    write_stream.write_all(&msg_bytes)?;
    info!("version: Sent");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn send_msg_verack(target_node: &SocketAddr, write_stream: &mut TcpStream) -> Result<()> {
    let payload = verack_msg();
    let msg = message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), payload);
    let msg_bytes = encode::serialize(&msg);

    info!(
        "verack: Sending {} bytes to {}",
        msg_bytes.len(),
        target_node
    );
    write_stream.write_all(&msg_bytes)?;
    info!("verack: Sent");

    Ok(())
}
