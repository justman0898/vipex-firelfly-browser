use crate::exceptions::custom_error::AppError;
use crate::models::scheme::Scheme;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Url {
    pub scheme: Scheme,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,

}

impl Url {
    pub fn parse(url: &str) -> Result<Self, AppError> {

        let(scheme_str, remainder) = url.split_once(":")
            .ok_or_else(|| AppError::InvalidUrl("Missing scheme".to_string()))?;

        let scheme = Scheme::parse(scheme_str)?;
        let mut remainder: &str = remainder;

        if remainder.starts_with("//") {
            remainder = &remainder[2..];
        }

        let (before_query, query) = match remainder.split_once('?') {
            Some((before, query)) => (before, Some(query.to_string())),
            None => (remainder, None),
        };

        let(host, port, path) = if Scheme::is_hierarchical(&scheme) {

            let (host_port, path) = match before_query.split_once('/') {
                Some((h, p)) => (h, format!("/{}", p)),
                None => (before_query, "/".to_string()),
            };

            if host_port.is_empty() {
                return Err(AppError::InvalidUrl("Invalid URL: Missing host".to_string()));
            }


            let (host, port) = match host_port.split_once(':') {
                Some((h, p)) => {
                    let parsed_port = p.parse::<u16>()
                        .map_err(|_| AppError::InvalidPort)?;
                    (Some(h.to_string()), Some(parsed_port))
                }
                None => (Some(host_port.to_string()), Scheme::default_port(&scheme)),
            };

            (host, port, path)

        }else {
            (None, None, before_query.to_string())
        };


        let url = Url{
            scheme,
            host,
            port,
            path: path.to_string(),
            query,
        };
        eprintln!("Url: {:?}", url);

        Ok(url)
    }


}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_that_panics_when_scheme_invalid(){
        let url = "uth://www.google.com";
        let result = Url::parse(url);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::InvalidScheme("uth".to_string())));
    }

    #[test]
    pub fn test_that_panics_when_url_invalid(){
        let url = "google.com";
        let result = Url::parse(url);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::InvalidUrl("Missing scheme".to_string())));
    }

    #[test]
    pub fn test_that_can_parse_valid_url(){
        let url = "https://example.com:8080/path?query=value";
        let result = Url::parse(url);
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.scheme, Scheme::Https);
        assert_eq!(result.host.unwrap(), "example.com".to_string());
        assert_eq!(result.port.unwrap(), 8080);
        assert_eq!(result.path, "/path".to_string());
        assert_eq!(result.query.unwrap(), "query=value".to_string());
    }

    #[test]
    pub fn test_that_default_port_is_used_when_port_is_missing(){
        let url = "https://example.com/path?query=value";
        let result = Url::parse(url);
        let result = result.unwrap();

        assert_eq!(result.port, Some(443));

        let url = "http://example.com/path?query=value";
        let result = Url::parse(url).unwrap();
        assert_eq!(result.port, Some(80));
    }

    #[test]
    pub fn test_that_can_handle_non_hierarchical_url(){
        let url = "about:blank";
        let result = Url::parse(url).unwrap();

        assert_eq!(result.host, None);
        assert_eq!(result.port, None);
        assert_eq!(result.path, "blank");
    }

    #[test]
    pub fn test_that_hierarchical_urls_must_have_host(){

        let url = "https:///index.html";
        let result = Url::parse(url);

        assert_eq!(result, Err(AppError::InvalidUrl("Invalid URL: Missing host".to_string())));


    }
}