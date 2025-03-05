use crate::{
    config::constants::ENCRYPTION_PREFIX,
    error::{Error, Result},
};

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};

use super::{derive_key, NONCE_SIZE};

pub fn decrypt_data(msg: &str, password: &str) -> Result<String> {
    let encoded = &msg[ENCRYPTION_PREFIX.len()..];

    log::trace!("Base64-decode the encrypted string");
    let combined_bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| Error::Custom(format!("failed to decode Base64: {}", e)))?;

    log::trace!("Ensure the combined data is long enough");
    if combined_bytes.len() < NONCE_SIZE {
        return Err(Error::Custom("Decryption failed".to_string()));
    }

    let nonce_bytes = &combined_bytes[..NONCE_SIZE];
    let ciphertext = &combined_bytes[NONCE_SIZE..];

    let key_bytes = derive_key(password);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    log::trace!("Decrypt the ciphertext using the extracted nonce");
    let plaintext_bytes = cipher
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|_| Error::Custom("Decryption failed".to_string()))?;

    log::trace!("Convert decrypted bytes to a UTF-8 string");
    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|_| Error::Custom("Decryption failed".to_string()))?;

    Ok(plaintext)
}
