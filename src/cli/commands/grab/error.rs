use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrabError {
    #[error("Key '{key}' could not be found")]
    KeyNotFound { key: String },
    #[error("Path '{path}' contains invalid UTF-8 characters")]
    InvalidPathString { path: String },
}
