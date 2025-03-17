use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrabError {
    #[error("Key {0} could not be foumd")]
    KeyNotFound(String),
}
