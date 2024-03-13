use hickory_resolver::{config::*, TokioAsyncResolver};
use std::{net::IpAddr, time::Duration};
use tracing::{error, info};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct DnsNameResolver {
    dns_seed_name: String,
    resolver: TokioAsyncResolver,
    timeout: Duration,
}

impl DnsNameResolver {
    pub fn new(dns_seed_name: String, timeout: Option<u64>) -> Self {
        DnsNameResolver {
            dns_seed_name,
            resolver: TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()),
            timeout: if let Some(t) = timeout {
                Duration::from_secs(t)
            } else {
                crate::handshake::FIVE_SECONDS
            },
        }
    }

    pub async fn resolve_names(&self) -> Vec<IpAddr> {
        let mut ip_list: Vec<IpAddr> = vec![];

        // Ensure supplied name ends with a dot
        let lookup_name = if self.dns_seed_name.ends_with(".") {
            self.dns_seed_name.clone()
        } else {
            format!("{}.", self.dns_seed_name)
        };

        // I can haz IP addresses?
        match tokio::time::timeout(self.timeout, self.resolver.lookup_ip(&lookup_name)).await {
            Ok(Ok(list_of_ips)) => {
                // START --> pretty tracing
                let ips = list_of_ips.clone();
                let mut count: u16 = 1;

                ips.into_iter().reduce(|_, addr| {
                    count += 1;
                    addr
                });

                info!(
                    "{} resolves to {} IP address{}\n",
                    self.dns_seed_name,
                    count,
                    if count == 1 { "" } else { "es" },
                );
                // END --> pretty tracing

                ip_list.extend(list_of_ips);
            }
            Ok(Err(e)) => error!("DNS Name resolution error: {}\n", e.kind()),
            Err(_) => error!(
                "DNS lookup of {} timed out after {} seconds",
                &self.dns_seed_name,
                self.timeout.as_secs()
            ),
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
    async fn should_resolve_name_with_static_ip() {
        let name_resolver = DnsNameResolver::new("www.whealy.com.".to_owned(), None);
        let response = name_resolver.resolve_names().await;

        assert_eq!(1, response.len());
        assert!(response[0].is_ipv4());
        assert_eq!(response[0], Ipv4Addr::new(141, 136, 43, 150));
    }

    #[tokio::test]
    async fn should_fail_to_resolve_nonexistent_name() {
        let name_resolver = DnsNameResolver::new("notthere.btc.petertodd.org.".to_owned(), None);
        let response = name_resolver.resolve_names().await;

        assert_eq!(0, response.len());
    }
}
