use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum AppError {

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Unsupported Scheme: {0}")]
    InvalidScheme(String),

    #[error("Invalid Port")]
    InvalidPort,

    #[error("DNS Error: {0}")]
    DnsError(String),
    
    #[error("DNS Lookup Error: No address found")]
    NoAddressesFound,

}