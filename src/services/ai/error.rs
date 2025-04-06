use crate::error::ParseError;

pub(crate) type Result<T> = std::result::Result<T, AiError>;

#[derive(thiserror::Error, Debug)]
pub enum AiError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse: {0}")]
    Parse(#[from] ParseError),
    #[error("Invalid JSON: {message}. Received: {json:#?}")]
    InvalidJson {
        json: serde_json::Value,
        message: String,
    },
    #[error("Validation error: {0}")]
    Validation(String),
}
