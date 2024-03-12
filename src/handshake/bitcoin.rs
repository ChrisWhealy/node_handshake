use bitcoin::{
    consensus::{encode, Decodable},
    p2p::message::{self},
};
use std::{
    io::{BufReader, Write},
    net::{IpAddr, SocketAddr, TcpStream},
};
use tracing::{error, info, warn};

use crate::{
    error::Result,
    handshake::{PORT_BITCOIN, TIMEOUT},
    messages::{verack::verack_msg, version::version_msg, PROTOCOL_VERSION},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn shake_my_hand(to_address: IpAddr) -> Result<()> {
    let mut msg_count: u8 = 0;
    let target_node = SocketAddr::new(to_address, PORT_BITCOIN);
    let ver_msg_payload = version_msg(target_node);

    info!("Connecting to {}:{}", to_address, PORT_BITCOIN);
    let mut write_stream = TcpStream::connect_timeout(&target_node, TIMEOUT)?;

    // Build and send version message
    let ver_msg =
        message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), ver_msg_payload);
    let msg_bytes = encode::serialize(&ver_msg);

    info!(
        "Version Message: Sending version {} ({} bytes) to target node {}",
        PROTOCOL_VERSION,
        msg_bytes.len(),
        to_address
    );
    write_stream.write_all(&msg_bytes)?;
    msg_count += 1;
    info!("Version Message: Sent");

    let read_stream = write_stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);

    // The protocol should be two-step receive loop
    loop {
        let some_response = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;

        // What did we get back?
        match some_response.payload() {
            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Target node responded with its version number
            message::NetworkMessage::Version(some_version) => {
                if some_version.version != PROTOCOL_VERSION {
                    let comparator = if some_version.version < PROTOCOL_VERSION {
                        "lower"
                    } else {
                        "higher"
                    };
                    warn!(
                        "Target node responded with {} version number {}",
                        comparator, some_version.version
                    )
                } else {
                    info!("Target node agreed on message version");
                }

                // Acknowledge target node's version message
                let verack_msg_bytes = verack_msg();
                let msg_bytes = encode::serialize(&verack_msg_bytes);

                info!(
                    "Version Acknowledgement Message: Sending {} bytes to {}",
                    msg_bytes.len(),
                    to_address
                );
                write_stream.write_all(&msg_bytes)?;
                msg_count += 1;
                info!("Version Acknowledgement Message: Sent");
            }

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Target node confirms version acknowledgement
            message::NetworkMessage::Verack => {
                // Did we get an early verack?
                if msg_count != 2 {
                    error!("Protocol violation: Target node sent VERACK before VERSION message");
                } else {
                    info!("Target node acknowledges version number");
                }

                break;
            }

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Target node respond with some other message
            _ => {
                if msg_count == 2 {
                    info!(
                        "Protocol violation (non-fatal): Expected VERACK, instead got {:?}",
                        some_response.payload()
                    );
                } else {
                    error!("Received unknown message {:?}", some_response.payload());
                }
                break;
            }
        }
    }

    write_stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

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
