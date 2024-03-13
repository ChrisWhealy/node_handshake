use bitcoin::{
    consensus::Decodable,
    p2p::{
        message::{self},
        PROTOCOL_VERSION,
    },
};
use std::{
    io::BufReader,
    net::{IpAddr, SocketAddr, TcpStream},
    time::{Duration, SystemTime},
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

    // Build and send version message
    send_msg_version(&target_node, &mut write_stream)?;

    let read_stream = write_stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);

    // The target node almost always responds correctly to a VERSION message; however, a successful response can
    // sometimes be delayed by over 2 minutes.
    // Wrapping the call to message::RawNetworkMessage::consensus_decode() inside a future::ready() then placing that
    // inside a call to tokio::time::timeout() does not trap a delayed response
    let then = SystemTime::now();
    let response1 = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;
    let decode_millis = SystemTime::now().duration_since(then).unwrap().as_millis();

    // Report decode times that exceed the timeout
    if decode_millis > (timeout * 1000) as u128 {
        warn!("Message decoding took {} ms", decode_millis);
    }
    // let response1 = if let Ok(net_msg) = tokio::time::timeout(
    //     Duration::from_secs(timeout),
    //     future::ready(message::RawNetworkMessage::consensus_decode(
    //         &mut stream_reader,
    //     )?),
    // )
    // .await
    // {
    //     net_msg
    // } else {
    //     return Err(CustomError(format!(
    //         "TIMEOUT: {} failed to respond with VERSION message within {} seconds",
    //         to_address, timeout
    //     )));
    // };

    // I can haz version message?
    match response1.payload() {
        // Is the target node compatible with our version?
        message::NetworkMessage::Version(some_ver_msg) => {
            if some_ver_msg.version < PROTOCOL_VERSION {
                error!(
                    "VERSION mismatch: Target node version {} too low.  Needs to be >= {}",
                    some_ver_msg.version, PROTOCOL_VERSION
                )
            } else {
                info!(
                    "VERSION: Target node accepts messages up to version {}",
                    some_ver_msg.version
                );
            }
        }

        // If we get an early verack, then throw toys out of pram
        message::NetworkMessage::Verack => {
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

    // I can haz verack message?
    match response2.payload() {
        message::NetworkMessage::Verack => info!("VERACK received"),

        _ => warn!(
            "Target node skipped VERACK.  Instead got {:?}",
            response2.payload()
        ),
    }

    // By the time we get to here, the TCP stream has sometimes already shutdown; in which case, this is just a warning
    match write_stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => warn!("TCP stream shutdown failed: {}", e),
    }

    Ok(())
}
