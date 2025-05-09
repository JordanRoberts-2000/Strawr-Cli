use crate::{constants::CONFIG_HOME_ENV, error::IoError};

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error("Home directory could not be found. Please set the '{CONFIG_HOME_ENV}' environment variable.")]
    HomeDirNotFound,
}
