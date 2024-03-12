use futures::StreamExt;
use hickory_resolver::error::ResolveError;

use node_handshake::{dns_name_resolver::*, DEFAULT_SEED_NODES};

#[tokio::main]
async fn main() -> Result<(), ResolveError> {
    tracing_subscriber::fmt::init();

    let name_resolver = DnsNameResolver::new(DEFAULT_SEED_NODES);

    tracing::info!("Resolving DNS names");
    let response = name_resolver.resolve_names().await;

    response
        .for_each_concurrent(0, |address| async move {
            tracing::info!("Attempting handshake with {:?}", address);
        })
        .await;

    Ok(())
}
