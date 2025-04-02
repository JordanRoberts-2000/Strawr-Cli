use thiserror::Error;

use crate::{
    error::ParseError,
    services::crypto,
    utils::{clipboard::ClipboardError, keychain::KeyChainError},
};

#[derive(Error, Debug)]
pub enum GrabError {
    #[error("Key '{key}' could not be found")]
    KeyNotFound { key: String },
    #[error("Key should have a value but does't")]
    KeyValueMissing,
    #[error("Path '{path}' contains invalid UTF-8 characters")]
    InvalidPathString { path: String },
    #[error("Failed to parse: {0}")]
    Parse(#[from] ParseError),
    #[error("io error: {context}")]
    Io {
        context: String,
        source: std::io::Error,
    },
    #[error("Internal error occured: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("Internal error occured: {0}")]
    Keychain(#[from] KeyChainError),
    #[error("Internal error occured: {0}")]
    Clipboard(#[from] ClipboardError),
}
