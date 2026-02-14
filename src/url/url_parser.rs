use std::fmt::Error;
use crate::exceptions::custom_error::AppError;

pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: Option<String>,

}

impl Url {
    pub fn parse(url: &str) -> Result<Vec<&str>, Error> {
        let parts: Vec<&str> = url.split("://").collect();
        let scheme = parts[0];
        if scheme != "http" && scheme != "https" {
            return Err(AppError::InvalidUrl("Only "))
        }

        eprintln!("{:?}", parts);
        Ok(parts)
    }
}

#[cfg(test)]
mod tests {
    use std::panic::panic_any;
    use super::*;

    #[test]
    pub fn test_that_can_split_url() {
        let url = "https://www.google.com";
        let url = Url::parse(url);
        match url {
            Ok(url) => {
                assert!(url.len()==2)
            },
            Err(_) => todo!()
        }

    }

    #[test]
    pub fn test_that_panics_when_scheme_invalid(){
        let url = "uth://www.google.com";
        let result = Url::parse(url);
        assert!(result.is_err())
    }
}