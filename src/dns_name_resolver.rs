use std::net::IpAddr;

use futures::stream::LocalBoxStream;
use futures::{future, StreamExt};
use hickory_resolver::{config::*, TokioAsyncResolver};
use tracing::{error, info};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct DnsNameResolver<'a> {
    name_list: &'a [&'a str],
    resolver: TokioAsyncResolver,
}

impl<'a> DnsNameResolver<'a> {
    pub fn new(name_list: &'a [&'a str]) -> Self {
        DnsNameResolver {
            name_list,
            resolver: TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()),
        }
    }

    pub async fn resolve_names(&'a self) -> LocalBoxStream<'a, IpAddr> {
        futures::stream::iter(self.name_list.iter())
            .then(move |name| self.resolver.lookup_ip(*name))
            .filter(|response| match response {
                Ok(_) => future::ready(true),
                Err(e) => {
                    error!("DNS Name resolution error: {}", e.kind());
                    future::ready(false)
                }
            })
            .flat_map(|response| {
                let resp1 = response.clone().unwrap();
                let resp2 = resp1.clone();
                let q = resp2.query();
                let mut count: u16 = 0;
                resp1.into_iter().reduce(|first, _| {
                    count += 1;
                    first
                });
                info!("{} resolves to {} IP addresses", q.name(), count);

                futures::stream::iter(response.unwrap().into_iter())
            })
            .boxed_local()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    const TEST_GOOD_DNS_NAME: &[&str] = &["www.whealy.com"];
    const TEST_BAD_DNS_NAME: &[&str] = &["wibble.wobble.com"];

    #[tokio::test]
    async fn should_resolve_hostname_with_static_ip() {
        let name_resolver = DnsNameResolver::new(TEST_GOOD_DNS_NAME);
        let response = name_resolver.resolve_names().await;

        response
            .for_each_concurrent(None, |address| async move {
                println!("{}", address);
                assert!(address.is_ipv4());
                assert_eq!(address, Ipv4Addr::new(141, 136, 43, 150));
            })
            .await;
    }

    #[tokio::test]
    async fn should_fail_to_resolve_nonexistent_hostname() {
        let name_resolver = DnsNameResolver::new(TEST_BAD_DNS_NAME);
        let response = name_resolver.resolve_names().await;

        response
            .for_each_concurrent(None, |address| async move {
                println!("{}", address);
                assert!(address.is_ipv6());
                assert_eq!(address, Ipv4Addr::new(141, 136, 43, 150));
            })
            .await;
    }
}
