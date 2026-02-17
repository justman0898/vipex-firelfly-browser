use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum AppError {

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Unsupported Scheme: {0}")]
    InvalidScheme(String),
    
    #[error("Invalid Port")]
    InvalidPort,

}