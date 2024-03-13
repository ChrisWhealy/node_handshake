use hickory_resolver::{config::*, TokioAsyncResolver};
use std::net::IpAddr;
use tracing::{error, info};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct DnsNameResolver {
    dns_seed_name: String,
    resolver: TokioAsyncResolver,
}

impl DnsNameResolver {
    pub fn new(dns_seed_name: String) -> Self {
        DnsNameResolver {
            dns_seed_name,
            resolver: TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()),
        }
    }

    pub async fn resolve_names(&self) -> Vec<IpAddr> {
        let mut ip_list: Vec<IpAddr> = vec![];

        // Ensure supplied name ends with a dot
        let lookup_name = if self.dns_seed_name.ends_with(".") {
            format!("{}.", self.dns_seed_name)
        } else {
            self.dns_seed_name.clone()
        };

        match self.resolver.lookup_ip(lookup_name).await {
            Ok(list_of_ips) => {
                let ip1 = list_of_ips.clone();
                let ip2 = list_of_ips.clone();
                let q = ip1.query();
                let mut count: u16 = 1;

                ip2.into_iter().reduce(|_, addr| {
                    count += 1;
                    addr
                });

                info!(
                    "{} resolves to {} IP address{}\n",
                    q.name(),
                    count,
                    if count == 1 { "" } else { "es" },
                );

                ip_list.extend(list_of_ips);
            }
            Err(e) => error!("DNS Name resolution error: {}\n", e.kind()),
        }

        ip_list
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn should_resolve_hostname_with_static_ip() {
        let name_resolver = DnsNameResolver::new("www.whealy.com.".to_owned());
        let response = name_resolver.resolve_names().await;

        assert!(response[0].is_ipv4());
        assert_eq!(response[0], Ipv4Addr::new(141, 136, 43, 150));
    }

    #[tokio::test]
    async fn should_fail_to_resolve_nonexistent_hostname() {
        let name_resolver = DnsNameResolver::new("notthere.btc.petertodd.org.".to_owned());
        let response = name_resolver.resolve_names().await;

        assert_eq!(0, response.len());
    }
}
