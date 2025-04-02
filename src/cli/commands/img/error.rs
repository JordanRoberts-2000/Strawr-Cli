use crate::{
    services::{ai, img},
    utils::{clipboard::ClipboardError, keychain::KeyChainError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error("Internal error occured: {0}")]
    ImgService(#[from] img::ImgError),
    #[error("Internal error occured: {0}")]
    AiService(#[from] ai::AiError),
    #[error("Internal error occured: {0}")]
    Keychain(#[from] KeyChainError),
    #[error("Internal error occured: {0}")]
    Clipboard(#[from] ClipboardError),
    #[error("Internal error occured: {0}")]
    AltTag(String),
}
