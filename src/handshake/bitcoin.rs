use bitcoin::{
    consensus::Decodable,
    p2p::{
        message::{self},
        PROTOCOL_VERSION,
    },
};
use std::time::Duration;
use std::{
    io::BufReader,
    net::{IpAddr, SocketAddr, TcpStream},
};
use tracing::{error, info, warn};

use crate::{
    error::Result,
    handshake::send_message::{send_msg_verack, send_msg_version},
    Error::CustomError,
};

static PROTOCOL_VIOLATION_UNEXPECTED_VERACK: &str =
    "Fatal Protocol Violation: Target node sent VERACK before VERSION message";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn shake_my_hand(to_address: IpAddr, port: u16, timeout: u64) -> Result<()> {
    let target_node = SocketAddr::new(to_address, port);

    info!("Connecting to {}:{}", to_address, port);
    let mut write_stream = TcpStream::connect_timeout(&target_node, Duration::from_secs(timeout))?;

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Build and send version message
    send_msg_version(&target_node, &mut write_stream)?;

    let read_stream = write_stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);
    let response1 = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;

    // What did we get back?
    match response1.payload() {
        message::NetworkMessage::Version(some_version) => {
            // Does the target node understand?
            if some_version.version < PROTOCOL_VERSION {
                error!(
                    "VERSION mismatch: Target node at version {}.  Expected version >= {}",
                    some_version.version, PROTOCOL_VERSION
                )
            } else {
                info!(
                    "VERSION: Target node accepts messages up to version {}",
                    some_version.version
                );
            }
        }

        message::NetworkMessage::Verack => {
            // If we get an early verack, then throw toys out of pram
            return Err(CustomError(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned()));
        }

        _ => {
            let err_msg = format!(
                "Target node failed to respond with VERSION message.  Instead got {:?}",
                response1.payload()
            );
            return Err(CustomError(err_msg));
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Respond to target node's version message with a VERACK
    send_msg_verack(&target_node, &mut write_stream)?;

    let response2 = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;

    // What did we get back?
    match response2.payload() {
        message::NetworkMessage::Verack => info!("VERACK received"),

        _ => warn!(
            "Target node skipped VERACK.  Instead got {:?}",
            response2.payload()
        ),
    }

    write_stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
