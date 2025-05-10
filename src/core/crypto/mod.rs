mod decrypt;
mod encrypt;
mod error;

pub use constants::ENCRYPTION_PREFIX;
pub use decrypt::decrypt;
pub use encrypt::encrypt;
pub use error::CryptoError;

mod constants {
    pub const ENCRYPTION_PREFIX: &str = "ENC:";
    pub const NONCE_SIZE: usize = 12;
}

mod utils {
    use sha2::{Digest, Sha256};

    // Derives a 32-byte key from the password using SHA256.
    pub fn derive_key(password: &str) -> [u8; 32] {
        let digest = Sha256::digest(password.as_bytes());
        digest.into()
    }
}
