use sha2::{Digest, Sha256};

pub mod decrypt_data;
pub mod encrypt_data;
pub mod get_or_prompt_keyring;

pub use decrypt_data::decrypt_data;
pub use encrypt_data::encrypt_data;
pub use get_or_prompt_keyring::get_or_prompt_keyring;

pub const NONCE_SIZE: usize = 12;

// Derives a 32-byte key from the password using SHA256.
pub fn derive_key(password: &str) -> [u8; 32] {
    let digest = Sha256::digest(password.as_bytes());
    digest.into()
}
