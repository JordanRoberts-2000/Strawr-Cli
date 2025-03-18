use thiserror::Error;

pub type Result<T> = std::result::Result<T, KeychainError>;

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("failed to read password: {0}")]
    ReadPasswordFailed(std::io::Error),
    #[error("{context}\n{source}")]
    Keyring {
        source: keyring::Error,
        context: String,
    },
}
