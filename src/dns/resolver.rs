use std::net::{IpAddr, SocketAddr};
use async_trait::async_trait;
use simple_logger::SimpleLogger;
use log::info;
use crate::exceptions::custom_error::AppError;

#[async_trait]
pub trait DnsResolver: Send + Sync {
    async fn resolve(
        &self,
        host: &str,
        port: u16,
    )-> Result<Vec<SocketAddr>, AppError>;
}

#[derive(Debug)]
pub struct BasicDnsResolver;

#[async_trait]
impl DnsResolver for BasicDnsResolver {
    async fn resolve(&self, host: &str, port: u16) -> Result<Vec<SocketAddr>, AppError> {

        let query = format!("{}:{}", host, port);

        let addresses = tokio::net::lookup_host(query)
            .await
            .map_err(|e| AppError::DnsError(format!("{}", e)))?;

        let mut ipv4: Vec<SocketAddr> = Vec::new();
        let mut ipv6: Vec<SocketAddr> = Vec::new();

        for address in addresses {
            match address.ip() {
                IpAddr::V4(_) => ipv4.push(address),
                IpAddr::V6(_) => ipv6.push(address),
            }
        }

        if ipv4.is_empty() && ipv6.is_empty() {
            return Err(AppError::NoAddressesFound)
        }

        ipv4.extend(ipv6);

        SimpleLogger::new().init().unwrap();
        info!("Addresses: {:?}", ipv4);
        Ok(ipv4)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_can_resolve_valid_host_port(){
        let host = "google.com";
        let port = 43u16;

        let resolver = BasicDnsResolver {};
        let result = resolver.resolve(host, port).await;

        assert!(result.is_ok());

        let addresses = result.unwrap();

        assert!(!addresses.is_empty());

        for address in addresses {
            assert_eq!(address.port(), port);
        }
    }

    #[tokio::test]
    async fn test_exception_is_thrown_invalid_host_port(){
        let host = "invalidhost";
        let port = 8444u16;

        let resolver = BasicDnsResolver {};
        let result = resolver.resolve(host, port).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::DnsError(_))));
    }
}