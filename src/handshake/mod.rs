pub mod bitcoin;
pub mod send_message;

use crate::{dns_name_resolver::*, error::Result, handshake::bitcoin::shake_my_hand};
use std::{fmt::Display, net::IpAddr, time::Duration};
use tracing::{error, info};

pub const PORT_BITCOIN: u16 = 8333;
pub static FIVE_SECONDS: Duration = Duration::from_secs(5);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq)]
pub enum HandshakeState {
    NotStarted,
    Started,
    Success,
    Failed(String),
    PartiallyComplete(String),
}

impl Display for HandshakeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeState::NotStarted => write!(f, "{}", "NOT STARTED".to_owned()),
            HandshakeState::Started => write!(f, "{}", "STARTED".to_owned()),
            HandshakeState::Success => write!(f, "{}", "SUCCESS".to_owned()),
            HandshakeState::Failed(reason) => write!(f, "{}", format!("FAILED: {}", reason)),
            HandshakeState::PartiallyComplete(reason) => {
                write!(f, "{}", format!("PARTIAL: {}", reason))
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct Handshake {
    dns_name: String,
    ip_addr: IpAddr,
    port: u16,
    pub state: HandshakeState,
}

impl Handshake {
    fn new(dns_name: String, ip_addr: IpAddr, port: u16) -> Handshake {
        Handshake {
            dns_name,
            ip_addr,
            port,
            state: HandshakeState::NotStarted,
        }
    }
}

impl Display for Handshake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Handshake with {} ({:?}) -> {}",
            self.dns_name, self.ip_addr, self.state
        )
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn start_handshakes(dns_seed_node: String, port: u16, timeout: u64) -> Result<()> {
    let mut count_success: u16 = 0;
    let mut count_partial: u16 = 0;
    let mut count_failure: u16 = 0;

    let name_resolver = DnsNameResolver::new(dns_seed_node.clone(), Some(timeout));
    let ip_address_list = name_resolver.resolve_names().await;

    for ip_addr in ip_address_list {
        let mut this_handshake = Handshake::new(dns_seed_node.clone(), ip_addr, port);
        info!("{}", this_handshake);

        match shake_my_hand(&mut this_handshake, timeout).await {
            Ok(_) => {
                if this_handshake.state == HandshakeState::Success {
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
