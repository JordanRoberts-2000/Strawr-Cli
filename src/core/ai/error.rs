use crate::{error::ParseError, validation::ValidationError};

pub(crate) type AiResult<T> = std::result::Result<T, AiError>;

#[derive(thiserror::Error, Debug)]
pub enum AiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("non-success HTTP status {status} from {url}")]
    HttpStatus {
        status: reqwest::StatusCode,
        url: String,
    },

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error("no content returned in choices: {0}")]
    NoContentReturned(serde_json::Value),

    #[error("no images returned in `data`; raw JSON:\n{0}")]
    NoImagesReturned(serde_json::Value),

    #[error("size `{size}` is not supported for model `{model}`")]
    UnsupportedSize { size: String, model: String },

    #[error("invalid image size `{0}`, must be one of 'sm', 'md', 'lg', 'tall', 'wide', or WxH")]
    ParseImageSize(String),
}
