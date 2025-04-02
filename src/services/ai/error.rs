pub(crate) type Result<T> = std::result::Result<T, AiError>;

#[derive(thiserror::Error, Debug)]
pub enum AiError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid JSON: {message}. Received: {json:#?}")]
    InvalidJson {
        json: serde_json::Value,
        message: String,
    },
}
