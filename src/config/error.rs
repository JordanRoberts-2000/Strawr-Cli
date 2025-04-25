use crate::error::{utils::format_validation_errors, IoError, ParseError};

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Failed to validate config file:\n{}", format_validation_errors(.0))]
    Validation(validator::ValidationErrors),
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] ParseError),
}
