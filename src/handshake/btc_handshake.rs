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

impl BitcoinHandshakeState {
    pub fn short_txt(&self) -> String {
        match self {
            BitcoinHandshakeState::NotStarted => format!("{}", "NOT STARTED"),
            BitcoinHandshakeState::Started => format!("{}", "STARTED"),
            BitcoinHandshakeState::Success => format!("{}", "SUCCESS"),
            BitcoinHandshakeState::Failed(_) => format!("{}", "FAILED"),
            BitcoinHandshakeState::PartiallyComplete(_) => format!("{}", "PARTIAL"),
        }
    }
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

    pub fn short_txt(&self) -> String {
        format!(
            "Handshake with {} ({:?}) -> {}",
            self.dns_name,
            self.ip_addr,
            self.state.short_txt()
        )
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
