use std::net::IpAddr;

use futures::stream::LocalBoxStream;
use futures::{future, StreamExt};
use hickory_resolver::{config::*, TokioAsyncResolver};

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
            .filter(|response| future::ready(response.is_ok()))
            .flat_map(|response| futures::stream::iter(response.unwrap().into_iter()))
            .boxed_local()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    const TEST_DNS_NAMES: &[&str] = &["www.whealy.com"];

    #[tokio::test]
    async fn should_resolve_hostname_with_static_ip() {
        let name_resolver = DnsNameResolver::new(TEST_DNS_NAMES);
        let response = name_resolver.resolve_names().await;

        response
            .for_each_concurrent(0, |address| async move {
                assert!(address.is_ipv4());
                assert_eq!(address, Ipv4Addr::new(141, 136, 43, 150));
            })
            .await;
    }
}
