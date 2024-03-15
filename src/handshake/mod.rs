mod bitcoin_handshake;
mod send_message;

use crate::{
    dns_name_resolver::*,
    error::Result,
    handshake::{
        bitcoin_handshake::{BitcoinHandshake, BitcoinHandshakeState},
        send_message::{send_msg_verack, send_msg_version},
    },
    Error::CustomError,
};

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
    time::Duration,
    time::SystemTime,
};
use tracing::{error, info, warn};

pub const PORT_BITCOIN: u16 = 8333;
pub static FIVE_SECONDS: Duration = Duration::from_secs(5);

static PROTOCOL_VIOLATION_UNEXPECTED_VERACK: &str =
    "Fatal Protocol Violation: Target node sent verack before version message";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn start_handshakes(dns_seed_node: String, port: u16, timeout: u64) -> Result<()> {
    let mut count_success: u16 = 0;
    let mut count_partial: u16 = 0;
    let mut count_failure: u16 = 0;

    let name_resolver = DnsNameResolver::new(dns_seed_node.clone(), Some(timeout));
    let ip_address_list = name_resolver.resolve_names().await;

    for ip_addr in ip_address_list {
        let mut this_handshake = BitcoinHandshake::new(dns_seed_node.clone(), ip_addr, port);
        info!("{}", this_handshake);

        match shake_my_hand(&mut this_handshake, timeout).await {
            Ok(_) => {
                if this_handshake.state == BitcoinHandshakeState::Success {
                    count_success += 1;
                } else {
                    count_partial += 1;
                }

                info!("{}\n", this_handshake)
            }
            Err(_) => {
                count_failure += 1;
                error!("{}\n", this_handshake)
            }
        }
    }

    info!("Summary of handshakes to {}", dns_seed_node);
    info!("   Success = {}", count_success);
    info!("   Partial = {}", count_partial,);
    info!("   Failed  = {}\n", count_failure);

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn shake_my_hand(handshake: &mut BitcoinHandshake, timeout: u64) -> Result<()> {
    handshake.state = BitcoinHandshakeState::Started;
    info!("{}", handshake);

    let target_node = SocketAddr::new(handshake.ip_addr, handshake.port);

    let mut write_stream =
        match TcpStream::connect_timeout(&target_node, Duration::from_secs(timeout)) {
            Ok(ws) => ws,
            Err(e) => {
                let err_msg = format!("TCP {}", e);
                handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
                return Err(CustomError(err_msg));
            }
        };

    // Let's assume everything works...
    handshake.state = BitcoinHandshakeState::Success;

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
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
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
                handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
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
                BitcoinHandshakeState::Failed(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned());
            return Err(CustomError(PROTOCOL_VIOLATION_UNEXPECTED_VERACK.to_owned()));
        }

        _ => {
            let err_msg = format!(
                "Target node failed to respond with version message.  Instead got {:?}",
                response1.payload()
            );
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
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
            handshake.state = BitcoinHandshakeState::PartiallyComplete(msg.clone());
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
