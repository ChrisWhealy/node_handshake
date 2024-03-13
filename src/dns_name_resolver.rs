use std::net::IpAddr;

use futures::{future, stream::LocalBoxStream, StreamExt};
use hickory_resolver::{
    lookup_ip::LookupIp,
    {config::*, TokioAsyncResolver},
};
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

    pub async fn resolve_names(&'a self) -> Vec<IpAddr> {
        let mut ip_list: Vec<IpAddr> = vec!();

        for dns_name in self.name_list {
            match self.resolver.lookup_ip(*dns_name).await {
                Ok(list_of_ips) => {
                    self.pretty_trace(list_of_ips.clone());
                    ip_list.extend(list_of_ips);
                }
                Err(e) => error!("DNS Name resolution error: {}", e.kind())
            }
        }

        ip_list
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    fn pretty_trace(&'a self, response: LookupIp) {
        let ip1 = response.clone();
        let q = ip1.query();
        let mut count: u16 = 1;

        response.into_iter().reduce(|_, addr| { count += 1; addr });

        info!(
            "{} resolves to {} IP address{}",
            q.name(),
            count,
            if count == 1 { "" } else { "es" },
        );
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
