use colored::*;
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ParseError {
    Toml(toml::de::Error),
    Json(serde_json::Error),
}

#[derive(Debug)]
pub enum Error {
    Validation(validator::ValidationErrors, String),
    Io(io::Error, String),
    Parse(ParseError, String),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err, msg) => {
                write!(f, "{}", format!("Io Error: {}, {}", msg, err).red())
            }
            Error::Parse(e, msg) => match e {
                ParseError::Toml(e) => write!(f, "TOML parse error: {}, {}", msg, e),
                ParseError::Json(e) => write!(f, "JSON parse error: {}, {}", msg, e),
            },
            Error::Validation(errors, title) => {
                // Iterate through each field error and format it.
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
                write!(f, "Validation error: {} \n{}", title, msgs.join("\n"))
            }
            Error::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
