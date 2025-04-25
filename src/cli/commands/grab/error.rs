use thiserror::Error;

use crate::{
    error::{IoError, ParseError},
    services::crypto,
    utils::{clipboard::ClipboardError, keyring::KeyringError},
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
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("Internal error occured: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("Internal error occured: {0}")]
    Keyring(#[from] KeyringError),
    #[error("Internal error occured: {0}")]
    Clipboard(#[from] ClipboardError),
}
