use crate::exceptions::custom_error::AppError;
use crate::models::scheme::Scheme::{About, File, Http, Https, Ws, Wss};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Scheme {
    Http,
    Https,
    Ws,
    About,
    Wss,
    File
}

impl Scheme {
     pub(crate) fn parse(input: &str) ->Result<Self, AppError>{
        match input {
            "http" => Ok(Http),
            "https" => Ok(Https),
            "ws" => Ok(Ws),
            "about" => Ok(About),
            "wss" => Ok(Wss),
            "file" => Ok(File),
            _ => Err(AppError::InvalidScheme(input.to_string()))
        }

    }

    pub(crate) fn default_port(&self) -> Option<u16> {
        match self {
            Scheme::Http | Scheme::Ws => Some(80),
            Scheme::Https | Scheme::Wss => Some(443),
            _ => None,
        }
    }
    
    pub(crate) fn is_hierarchical(&self) -> bool {
        matches!(self, 
            Scheme::File | Scheme::Http | Scheme::Wss | Scheme::Ws | Scheme::Https
        )        
    }

}