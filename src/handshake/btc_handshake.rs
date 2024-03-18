use std::{fmt::Display, net::IpAddr, time::Duration};

use super::FIVE_SECONDS;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq)]
pub enum BitcoinHandshakeState {
    NotStarted,
    Started,
    Success,
    Failed(String),
    PartiallyComplete(String),
}

impl Display for BitcoinHandshakeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitcoinHandshakeState::NotStarted => write!(f, "{}", "NOT STARTED".to_owned()),
            BitcoinHandshakeState::Started => write!(f, "{}", "STARTED".to_owned()),
            BitcoinHandshakeState::Success => write!(f, "{}", "SUCCESS".to_owned()),
            BitcoinHandshakeState::Failed(reason) => write!(f, "{}", format!("FAILED: {}", reason)),
            BitcoinHandshakeState::PartiallyComplete(reason) => {
                write!(f, "{}", format!("PARTIAL: {}", reason))
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct BitcoinHandshake {
    pub dns_name: String,
    pub ip_addr: IpAddr,
    pub port: u16,
    pub state: BitcoinHandshakeState,
    pub timeout_millis: Duration,
}

impl BitcoinHandshake {
    pub fn new(
        dns_name: String,
        ip_addr: IpAddr,
        port: u16,
        timeout_millis: Option<Duration>,
    ) -> BitcoinHandshake {
        BitcoinHandshake {
            dns_name,
            ip_addr,
            port,
            state: BitcoinHandshakeState::NotStarted,
            timeout_millis: if let Some(duration) = timeout_millis {
                duration
            } else {
                FIVE_SECONDS
            },
        }
    }
}

impl Display for BitcoinHandshake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Handshake with {} ({:?}) -> {}",
            self.dns_name, self.ip_addr, self.state
        )
    }
}
