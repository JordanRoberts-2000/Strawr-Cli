use std::path::PathBuf;

use thiserror::Error;

use crate::{
    ai::AiError,
    img::ImgError as ImageError,
    services::{errors::ClipboardError, keyring::KeyringError},
};

#[derive(Error, Debug)]
pub enum ImgError {
    #[error("Invalid input, please provide valid file, directory or remote-url: {0}")]
    UnknownInputType(String),
    #[error("Input not found: the specified path does not exist: '{}'", .0.display())]
    InputNotFound(PathBuf),
    #[error("Input url is not a remote-url: {0}")]
    InputUrlNotRemote(String),
    #[error("Internal error occured: {0}")]
    ImgService(#[from] ImageError),
    #[error("Internal error occured: {0}")]
    AiService(#[from] AiError),
    #[error("Internal error occured: {0}")]
    Keyring(#[from] KeyringError),
    #[error("Internal error occured: {0}")]
    Clipboard(#[from] ClipboardError),
    #[error("Internal error occured: {0}")]
    AltTag(String),
    #[error("ValidImageFormat::Original does not map to a concrete ImageFormat")]
    NoConcreteFormat,
    #[error("invalid size: {0}")]
    ParseImageSize(String),
}
