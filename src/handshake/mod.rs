mod btc_handshake;
mod btc_message;

use crate::{
    dns_name_resolver::*,
    error::Result,
    handshake::{
        btc_handshake::{BitcoinHandshake, BitcoinHandshakeState},
        btc_message::{handle_verack_msg, handle_version_msg},
    },
    Error::CustomError,
};

use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};
use tracing::{error, info, warn};

pub const PORT_BITCOIN: u16 = 8333;
pub static FIVE_SECONDS: Duration = Duration::from_millis(5000);

const USER_AGENT: &str = "P2P Handshake PoC";
const PROTOCOL_VIOLATION_UNEXPECTED_VERACK: &str =
    "Fatal Protocol Violation: Target node sent verack before version message";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn start_handshakes(dns_seed_node: String, port: u16, timeout_millis: u64) -> Result<()> {
    let mut count_success: u16 = 0;
    let mut count_partial: u16 = 0;
    let mut count_failure: u16 = 0;
    let timeout = Duration::from_millis(timeout_millis);

    let name_resolver = DnsNameResolver::new(dns_seed_node.clone(), timeout);

    if let Some(ip_address_list) = name_resolver.resolve_names().await {
        for ip_addr in ip_address_list {
            let mut this_handshake =
                BitcoinHandshake::new(dns_seed_node.clone(), ip_addr, port, Some(timeout));
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
    } else {
        error!("Unable to start hand shakes\n");
    };

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn shake_my_hand(handshake: &mut BitcoinHandshake, timeout: Duration) -> Result<()> {
    handshake.state = BitcoinHandshakeState::Started;
    info!("{}", handshake);

    let target_node = SocketAddr::new(handshake.ip_addr, handshake.port);

    let mut write_stream = match TcpStream::connect_timeout(&target_node, timeout) {
        Ok(ws) => ws,
        Err(e) => {
            let err_msg = format!("TCP {}", e);
            handshake.state = BitcoinHandshakeState::Failed(err_msg.clone());
            return Err(CustomError(err_msg));
        }
    };

    // Let's assume the handshake works...
    handshake.state = BitcoinHandshakeState::Success;

    handle_version_msg(&target_node, &mut write_stream, handshake).await?;
    handle_verack_msg(&target_node, &mut write_stream, handshake).await?;

    // Don't worry if the TCP stream has already shutdown...
    if let Err(e) = write_stream.shutdown(std::net::Shutdown::Both) {
        warn!("TCP stream shutdown failed: {}", e);
    }

    Ok(())
}
