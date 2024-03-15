use bitcoin::{
    consensus::Decodable,
    p2p::{
        message::{self},
        PROTOCOL_VERSION,
    },
};
use std::{
    io::BufReader,
    net::{SocketAddr, TcpStream},
    time::{Duration, SystemTime},
};
use tracing::{info, warn};

use crate::{
    error::Result,
    handshake::{
        send_message::{send_msg_verack, send_msg_version},
        Handshake, HandshakeState,
    },
    Error::CustomError,
};

static PROTOCOL_VIOLATION_UNEXPECTED_VERACK: &str =
    "Fatal Protocol Violation: Target node sent verack before version message";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn shake_my_hand(handshake: &mut Handshake, timeout: u64) -> Result<()> {
    handshake.state = HandshakeState::Started;
    info!("{}", handshake);

    let target_node = SocketAddr::new(handshake.ip_addr, handshake.port);

    let mut write_stream =
        match TcpStream::connect_timeout(&target_node, Duration::from_secs(timeout)) {
            Ok(ws) => ws,
            Err(e) => {
                let err_msg = format!("TCP {}", e);
                handshake.state = HandshakeState::Failed(err_msg.clone());
                return Err(CustomError(err_msg));
            }
        };

    // Let's assume everything works...
    handshake.state = HandshakeState::Success;

    // Build and send version message
    send_msg_version(&target_node, &mut write_stream)?;

    let read_stream = write_stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);

    // The target node almost always responds correctly to a version message; however, a successful response can
    // sometimes be significantly delayed.  See https://stackoverflow.com/questions/78156419/ for possible solution
    let then = SystemTime::now();
    let response1 = match message::RawNetworkMessage::consensus_decode(&mut stream_reader) {
        Ok(raw_net_msg) => raw_net_msg,
        Err(e) => {
            let err_msg = format!("Bitcoin encoding error: {}", e);
            handshake.state = HandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }
    };
    let decode_millis = SystemTime::now().duration_since(then).unwrap().as_millis();

    // Report message arrival times that exceed the timeout
    if decode_millis > (timeout * 1000) as u128 {
        warn!("version: Message took {} ms to arrive", decode_millis);
    }

    // I can haz version message?
    match response1.payload() {
        // Is the target node compatible with our version?
        message::NetworkMessage::Version(some_ver_msg) => {
            if some_ver_msg.version < PROTOCOL_VERSION {
                let err_msg = format!(
                    "version mismatch: Target node version {} too low.  Needs to be >= {}",
                    some_ver_msg.version, PROTOCOL_VERSION
                );
                handshake.state = HandshakeState::Failed(err_msg.clone());
                return Err(CustomError(err_msg));
            } else {
                info!(
                    "version: Target node accepts messages up to version {}",
                    some_ver_msg.version
                );
            }
        }

        // If we get an early verack, then throw toys out of pram
        message::NetworkMessage::Verack => {
            handshake.state =
                HandshakeState::Failed(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned());
            return Err(CustomError(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned()));
        }

        _ => {
            let err_msg = format!(
                "Target node failed to respond with version message.  Instead got {:?}",
                response1.payload()
            );
            handshake.state = HandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Respond to target node's version message with a verack
    send_msg_verack(&target_node, &mut write_stream)?;

    let response2 = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;

    // I can haz verack message?
    match response2.payload() {
        message::NetworkMessage::Verack => info!("verack received"),

        _ => {
            let msg = format!(
                "Target node skipped verack.  Instead got {:?}",
                response2.payload()
            );
            handshake.state = HandshakeState::PartiallyComplete(msg.clone());
            warn!("{}", msg)
        }
    }

    // By the time we get to here, the TCP stream has sometimes already shutdown; in which case, this is just a warning
    match write_stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => warn!("TCP stream shutdown failed: {}", e),
    }

    Ok(())
}
