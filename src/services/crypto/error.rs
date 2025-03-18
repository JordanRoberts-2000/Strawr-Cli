use thiserror::Error;

pub type Result<T> = std::result::Result<T, CryptoError>;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Decryption failed")]
    Decryption,
    #[error("An internal error occured")]
    Internal,
}
