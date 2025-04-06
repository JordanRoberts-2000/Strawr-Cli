use thiserror::Error;

use crate::{
    error::ParseError,
    services::crypto,
    utils::{clipboard::ClipboardError, keychain::KeyChainError},
};

#[derive(Error, Debug)]
pub enum GrabError {
    #[error("Key '{0}' could not be found")]
    KeyNotFound(String),
    #[error("No keys found. Try adding one first.")]
    NoKeysAvailable,
    #[error("Failed to retrieve key")]
    FailedToRetrieveKey(#[from] inquire::InquireError),
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
