pub mod bitcoin;
pub mod send_message;

use crate::{
    dns_name_resolver::*, error::Result, handshake::bitcoin::shake_my_hand, DEFAULT_SEED_NODES,
};
use futures::StreamExt;
use std::time::Duration;
use tracing::{error, info};

pub const PORT_BITCOIN: u16 = 8333;
pub static TIMEOUT: Duration = Duration::from_secs(5);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn start_handshakes() -> Result<()> {
    let name_resolver = DnsNameResolver::new(DEFAULT_SEED_NODES);

    tracing::info!("Resolving DNS names");
    let resolved_names = name_resolver.resolve_names().await;

    for address in resolved_names {
        info!("Attempting handshake with {:?}:{}", address, PORT_BITCOIN);

        match shake_my_hand(address).await {
            Ok(_) => info!("Handshake with {:?} succeeded\n", address),
            Err(e) => error!("Handshake with {:?} failed: {}\n", address, e),
        }
    }

    Ok(())
}
