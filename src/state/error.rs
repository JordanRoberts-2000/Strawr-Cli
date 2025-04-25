use crate::error::IoError;

#[derive(thiserror::Error, Debug)]
pub enum StateError {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("Home directory could not be found. Please set the {env_var} environment variable.")]
    HomeDirNotFound { env_var: &'static str },
}
