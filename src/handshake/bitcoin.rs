use bitcoin::{
    consensus::Decodable,
    p2p::message::{self},
};
use std::{
    io::BufReader,
    net::{IpAddr, SocketAddr, TcpStream},
};
use tracing::{info, warn};

use crate::{
    error::Result,
    handshake::{
        send_message::{send_msg_verack, send_msg_version},
        PORT_BITCOIN, TIMEOUT,
    },
    messages::PROTOCOL_VERSION,
    Error::CustomError,
};

static PROTOCOL_VIOLATION_UNEXPECTED_VERACK: &str =
    "Protocol violation: Target node sent VERACK before VERSION message";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn shake_my_hand(to_address: IpAddr) -> Result<()> {
    let target_node = SocketAddr::new(to_address, PORT_BITCOIN);

    info!("Connecting to {}:{}", to_address, PORT_BITCOIN);
    let mut write_stream = TcpStream::connect_timeout(&target_node, TIMEOUT)?;

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Build and send version message
    send_msg_version(&target_node, &mut write_stream)?;

    let read_stream = write_stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);
    let response1 = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;

    // What did we get back?
    match response1.payload() {
        message::NetworkMessage::Version(some_version) => {
            // Are we agreed on the message version?
            if some_version.version != PROTOCOL_VERSION {
                warn!(
                    "VERSION mismatch: Target node at {}, expected {}",
                    some_version.version, PROTOCOL_VERSION
                )
            } else {
                info!("VERSION: Target node agreed on message version");
            }
        }

        message::NetworkMessage::Verack => {
            // If we get an early verack, then throw toys out of pram
            return Err(CustomError(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned()));
        }

        _ => {
            warn!(
                "Protocol violation (non-fatal): Expected VERSION, instead got {:?}",
                response1.payload()
            );
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
            "Protocol violation (non-fatal): Expected VERACK, instead got {:?}",
            response2.payload()
        ),
    }

    write_stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    use tracing::error;

    #[tokio::test]
    async fn should_perform_handshake_to_single_target_node() {
        let address = IpAddr::V4(Ipv4Addr::new(88, 98, 84, 206));

        if let Ok(_) = shake_my_hand(address).await {
            info!("Handshake with {:?} succeeded\n", address);
        } else {
            error!("Handshake with {:?} failed\n", address);
        };
    }
}
