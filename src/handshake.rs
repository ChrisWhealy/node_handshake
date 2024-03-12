pub mod bitcoin;

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

    resolved_names
        .for_each_concurrent(None, |address| async move {
            info!("Attempting handshake with {:?}", address);

            match shake_my_hand(address).await {
                Ok(_) => info!("Handshake with {:?} succeeded\n", address),
                Err(e) => error!("Handshake with {:?} failed: {}\n", address, e),
            }
        })
        .await;

    Ok(())
}
