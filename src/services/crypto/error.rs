pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to Base64-decode the encrypted string: {source}")]
    Base64Decode { source: base64::DecodeError },
    #[error("Encryption failed: {0}")]
    Encryption(aes_gcm::Error),
    #[error("Decryption failed: {0}")]
    Decryption(aes_gcm::Error),
    #[error("Decrypted data is too short: expected at least {expected} bytes, got {found}")]
    InvalidCiphertextLength { expected: usize, found: usize },
    #[error("Failed to convert decrypted bytes to a string: {source}")]
    Utf8Conversion { source: std::string::FromUtf8Error },
    #[error("Internal error: {message}")]
    Internal { message: String },
}
