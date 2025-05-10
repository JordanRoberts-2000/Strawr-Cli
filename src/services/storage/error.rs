use crate::{constants::CONFIG_HOME_ENV, error::IoError};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Home directory could not be found. Please set the '{CONFIG_HOME_ENV}' environment variable.")]
    HomeDirNotFound,

    #[error(transparent)]
    Io(#[from] IoError),
}
