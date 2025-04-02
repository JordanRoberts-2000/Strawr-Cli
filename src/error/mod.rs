pub use parse::ParseError;
use thiserror::Error;

use crate::{
    cli::commands::{grab::GrabError, img::ImgError},
    config::error::ConfigError,
    state::error::StateError,
};

pub type Result<T> = std::result::Result<T, Error>;

pub mod parse;
pub mod utils;

#[derive(Error, Debug)]
pub enum Error {
    // #[error(transparent)]
    #[error("[Error] {0}")]
    State(#[from] StateError),
    #[error("[Error] {0}")]
    Config(#[from] ConfigError),

    // Commands
    #[error("[Error]: {0}")]
    Grab(#[from] GrabError),
    #[error("[Error]: {0}")]
    Img(#[from] ImgError),
}
