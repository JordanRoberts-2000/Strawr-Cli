use crate::{
    services::{ai, img},
    utils::{clipboard::ClipboardError, keyring::KeyringError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error("Input type unknown")]
    UnknownInputType,
    #[error("Input not found: the specified path does not exist: {0:?}")]
    InputNotFound(String),
    #[error("Internal error occured: {0}")]
    ImgService(#[from] img::ImgError),
    #[error("Internal error occured: {0}")]
    AiService(#[from] ai::AiError),
    #[error("Internal error occured: {0}")]
    Keyring(#[from] KeyringError),
    #[error("Internal error occured: {0}")]
    Clipboard(#[from] ClipboardError),
    #[error("Internal error occured: {0}")]
    AltTag(String),
}
