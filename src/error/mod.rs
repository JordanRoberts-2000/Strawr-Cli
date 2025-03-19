use std::io;

pub use parse::ParseError;
use thiserror::Error;
use validation::format_validation_errors;

use crate::{
    cli::commands::grab::GrabError,
    services::{crypto::CryptoError, keychain::error::KeychainError},
};

pub type Result<T> = std::result::Result<T, Error>;

pub mod parse;
pub mod validation;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[Validation Error]: {context}:\n{}", format_validation_errors(.source))]
    Validation {
        source: validator::ValidationErrors,
        context: String,
    },
    #[error("[Io Error]: {context}\nCaused by: {source}")]
    Io { source: io::Error, context: String },
    #[error("[Parse Error]: {0}")]
    Parse(#[from] ParseError),
    #[error("[Error]: An internal error occurred")]
    Internal,
    #[error("[Error]: {0}")]
    Custom(String),

    // Services
    #[error("[Error]: {0}")]
    KeyChain(#[from] KeychainError),
    #[error("[Error]: {0}")]
    Crypto(#[from] CryptoError),

    // Commands
    #[error("[Error]: {0}")]
    Grab(#[from] GrabError),
}
