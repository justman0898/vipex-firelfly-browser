use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
pub enum AppError {

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

}