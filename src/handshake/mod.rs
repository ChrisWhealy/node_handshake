pub mod bitcoin;
pub mod send_message;

use crate::{dns_name_resolver::*, error::Result, handshake::bitcoin::shake_my_hand};
use std::time::Duration;
use tracing::{error, info};

pub const PORT_BITCOIN: u16 = 8333;
pub static FIVE_SECONDS: Duration = Duration::from_secs(5);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn start_handshakes(dns_seed_node: String, port: u16, timeout: u64) -> Result<()> {
    let name_resolver = DnsNameResolver::new(dns_seed_node.clone(), Some(timeout));
    let ip_address_list = name_resolver.resolve_names().await;

    for ip_addr in ip_address_list {
        info!("Attempting handshake with {:?}:{}", ip_addr, port);

        match shake_my_hand(ip_addr, port, timeout).await {
            Ok(_) => info!("Handshake with {:?}:{} succeeded\n", ip_addr, port),
            Err(e) => error!("Handshake with {:?}:{} failed: {}\n", ip_addr, port, e),
        }
    }

    Ok(())
}
