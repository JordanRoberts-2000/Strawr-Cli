use std::io;

pub use parse::ParseError;
use thiserror::Error;

use crate::cli::commands::grab::GrabError;

pub type Result<T> = std::result::Result<T, Error>;

pub mod parse;

fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    let msgs: Vec<String> = errors
        .field_errors()
        .iter()
        .flat_map(|(field, errs)| {
            errs.iter().map(move |err| {
                // Use a custom message if available, or a default message.
                let msg = err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "validation error".into());
                format!("Validation failed - {}: {}", field, msg)
            })
        })
        .collect();

    msgs.join("\n")
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("[Validation Error]: \n{}, {}", format_validation_errors(.0), .1)]
    Validation(validator::ValidationErrors, String),
    #[error("[Io Error]: {1}\nCaused by: {0}")]
    Io(io::Error, String),
    #[error("[Parse Error]: {0}")]
    Parse(#[from] ParseError),
    #[error("[Error]: An internal error occurred")]
    Internal,
    #[error("[Error]: {0}")]
    Custom(String),

    // Services
    #[error("[Keyring Error]: {1}, {0}")]
    Keyring(keyring::Error, String),

    // Commands
    #[error("[Error]: {0}")]
    Grab(#[from] GrabError),
}
