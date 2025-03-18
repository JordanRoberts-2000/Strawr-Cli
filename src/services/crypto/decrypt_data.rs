use super::{
    constants::{ENCRYPTION_PREFIX, NONCE_SIZE},
    error::{CryptoError, Result},
    utils::derive_key,
};

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};

pub fn decrypt_data(msg: &str, password: &str) -> Result<String> {
    log::trace!("Decryption attempted");

    let encoded = &msg[ENCRYPTION_PREFIX.len()..];

    let combined_bytes = general_purpose::STANDARD.decode(encoded).map_err(|e| {
        log::error!("Failed to Base64-decode the encrypted string, {}", e);
        CryptoError::Internal
    })?;

    if combined_bytes.len() < NONCE_SIZE {
        #[cfg(debug_assertions)]
        log::error!("Decryption failed due to size invalidation");
        return Err(CryptoError::Decryption);
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
            CryptoError::Decryption
        })?;

    let plaintext = String::from_utf8(plaintext_bytes).map_err(|_| {
        log::error!("Failed to turn decryped data from bytes to a string");
        CryptoError::Internal
    })?;

    log::trace!("Decryption successfull");
    Ok(plaintext)
}
