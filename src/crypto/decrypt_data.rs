use crate::{
    config::constants::ENCRYPTION_PREFIX,
    error::{Error, Result},
};

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};

use super::{derive_key, NONCE_SIZE};

pub fn decrypt_data(msg: &str, password: &str) -> Result<String> {
    log::trace!("Decryption attempted");

    let encoded = &msg[ENCRYPTION_PREFIX.len()..];

    let combined_bytes = general_purpose::STANDARD.decode(encoded).map_err(|e| {
        log::error!("Failed to Base64-decode the encrypted string, {}", e);
        Error::Internal
    })?;

    if combined_bytes.len() < NONCE_SIZE {
        #[cfg(debug_assertions)]
        log::error!("Decryption failed due to size invalidation");
        return Err(Error::Custom("Decryption failed".to_string()));
    }

    let nonce_bytes = &combined_bytes[..NONCE_SIZE];
    let ciphertext = &combined_bytes[NONCE_SIZE..];

    let key_bytes = derive_key(password);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let plaintext_bytes = cipher
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|e| {
            #[cfg(debug_assertions)]
            log::error!("Decryption failed: {}", e);
            Error::Custom("Decryption failed".to_string())
        })?;

    let plaintext = String::from_utf8(plaintext_bytes).map_err(|_| {
        log::error!("Failed to turn decryped data from bytes to a string");
        Error::Internal
    })?;

    log::trace!("Decryption successfull");
    Ok(plaintext)
}
