use crate::{
    error::{Error, Result},
    handshake::{
        btc_handshake::{BitcoinHandshake, BitcoinHandshakeState},
        PROTOCOL_VIOLATION_UNEXPECTED_VERACK, USER_AGENT,
    },
    Error::CustomError,
};

use bitcoin::{
    consensus::{encode, Decodable},
    p2p::{
        self, address,
        message::{self, RawNetworkMessage},
        message_network, PROTOCOL_VERSION,
    },
    secp256k1::rand::{self, Rng},
};
use std::{
    io::{BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{info, warn};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Send a generic BitCoin network message
fn send_msg(
    payload: message::NetworkMessage,
    target_node: &SocketAddr,
    write_stream: &mut TcpStream,
) -> Result<()> {
    let cmd = payload.cmd();
    let msg = message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), payload);
    let msg_bytes = encode::serialize(&msg);

    info!(
        "{}: Sending {} bytes to {}",
        &cmd,
        msg_bytes.len(),
        target_node
    );
    write_stream.write_all(&msg_bytes)?;
    info!("{}: Sent", &cmd);

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Wait for the remote node to respond with a BitCoin message
// This function is impatient and will mark a handshake as having failed if the response takes longer than the timeout
// period.  This is in spite of the fact had we waited long enough, the target node may well have responsed with a
// success message
pub async fn receive_msg(
    write_stream: &mut TcpStream,
    handshake: &mut BitcoinHandshake,
    cmd: &str,
) -> std::result::Result<RawNetworkMessage, crate::error::Error> {
    let read_stream = write_stream.try_clone()?;

    let maybe_raw_net_msg = tokio::time::timeout(
        handshake.timeout,
        tokio::task::spawn_blocking(move || {
            let mut stream_reader = BufReader::new(read_stream);
            message::RawNetworkMessage::consensus_decode(&mut stream_reader)
        }),
    )
    .await;

    // Unpack the triple nested Result...
    let raw_net_msg = match maybe_raw_net_msg {
        // Got a success response within the timeout period
        Ok(Ok(Ok(raw_net_msg))) => raw_net_msg,

        // Got some error within the timeout period
        Ok(Ok(Err(e))) => {
            handshake.state = BitcoinHandshakeState::Failed(e.to_string());
            return Err(Error::Encode(e));
        }

        // Got an error handling the timeout
        Ok(Err(e)) => {
            let err_msg = format!("Error handling timeout: {}", e);
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }

        // Remote server timed out
        Err(_e) => {
            let err_msg = format!("Timed out waiting for '{}' message from target node", cmd);
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }
    };

    Ok(raw_net_msg)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Build and send version message then handle response
pub async fn handle_version_msg(
    target_node: &SocketAddr,
    write_stream: &mut TcpStream,
    handshake: &mut BitcoinHandshake,
) -> Result<()> {
    let ignore_from_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error: Unable to get system time")
        .as_secs();
    let version_payload = message::NetworkMessage::Version(message_network::VersionMessage::new(
        Default::default(),
        now as i64,
        address::Address::new(&target_node, p2p::ServiceFlags::NONE),
        address::Address::new(&ignore_from_address, p2p::ServiceFlags::NONE),
        rand::thread_rng().gen(),
        USER_AGENT.to_owned(),
        0,
    ));

    send_msg(version_payload, &target_node, write_stream)?;

    let raw_net_msg = receive_msg(write_stream, handshake, "version").await?;

    // I can haz version message?
    match raw_net_msg.payload() {
        // Will the target node play nicely?
        message::NetworkMessage::Version(some_ver_msg) => {
            if some_ver_msg.version < PROTOCOL_VERSION {
                let err_msg = format!(
                    "version mismatch: Target node version {} too low.  Needs to be >= {}",
                    some_ver_msg.version, PROTOCOL_VERSION
                );
                handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
                return Err(CustomError(err_msg));
            } else {
                info!(
                    "version: Target node accepts messages up to version {}",
                    some_ver_msg.version
                );
            }
        }

        // If we get an early verack, then this is bad -> throw toys out of pram
        message::NetworkMessage::Verack => {
            handshake.state =
                BitcoinHandshakeState::Failed(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned());
            return Err(CustomError(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned()));
        }

        _ => {
            let err_msg = format!(
                "Target node failed to respond with version message.  Instead got {:?}",
                raw_net_msg.payload()
            );
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }
    }

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Build and send verack message then handle response
pub async fn handle_verack_msg(
    target_node: &SocketAddr,
    write_stream: &mut TcpStream,
    handshake: &mut BitcoinHandshake,
) -> Result<()> {
    send_msg(message::NetworkMessage::Verack, &target_node, write_stream)?;

    let raw_net_msg = receive_msg(write_stream, handshake, "verack").await?;

    // I can haz verack message?
    match raw_net_msg.payload() {
        message::NetworkMessage::Verack => info!("verack received"),

        _ => {
            let msg = format!(
                "Target node skipped verack.  Instead got {:?}",
                raw_net_msg.payload()
            );
            handshake.state = BitcoinHandshakeState::PartiallyComplete(msg.clone());
            warn!("{}", msg)
        }
    }

    Ok(())
}
