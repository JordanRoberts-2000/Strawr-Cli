use sha2::{Digest, Sha256};

// Derives a 32-byte key from the password using SHA256.
pub fn derive_key(password: &str) -> [u8; 32] {
    let digest = Sha256::digest(password.as_bytes());
    digest.into()
}
