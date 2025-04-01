use super::{
    constants::{ENCRYPTION_PREFIX, NONCE_SIZE},
    error::{Error, Result},
    utils::derive_key,
};

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};

pub fn decrypt(msg: &str, password: &str) -> Result<String> {
    // Remove the encryption prefix.
    let encoded = &msg[ENCRYPTION_PREFIX.len()..];

    // Attempt to decode the Base64 string.
    let combined_bytes = general_purpose::STANDARD.decode(encoded).map_err(|e| {
        log::error!("Failed to Base64-decode the encrypted string: {}", e);
        Error::Base64Decode { source: e }
    })?;

    // Verify that the combined data has at least enough bytes for the nonce.
    if combined_bytes.len() < NONCE_SIZE {
        log::error!(
            "Invalid ciphertext length: expected at least {} bytes, got {}",
            NONCE_SIZE,
            combined_bytes.len()
        );
        return Err(Error::InvalidCiphertextLength {
            expected: NONCE_SIZE,
            found: combined_bytes.len(),
        });
    }

    let nonce_bytes = &combined_bytes[..NONCE_SIZE];
    let ciphertext = &combined_bytes[NONCE_SIZE..];

    let key_bytes = derive_key(password);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    // Attempt decryption.
    let plaintext_bytes = cipher
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|e| {
            log::error!("Decryption failed: {}", e);
            Error::Decryption(e)
        })?;

    // Convert decrypted bytes into a UTF-8 string.
    let plaintext = String::from_utf8(plaintext_bytes).map_err(|e| {
        log::error!("Failed to convert decrypted bytes to a string: {}", e);
        Error::Utf8Conversion { source: e }
    })?;

    Ok(plaintext)
}
