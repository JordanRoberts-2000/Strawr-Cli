use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};
use rand::prelude::*;

use super::{
    constants::{ENCRYPTION_PREFIX, NONCE_SIZE},
    error::{CryptoError, Result},
    utils::derive_key,
};

// The output layout is: [prefix] + [nonce (12 bytes)][ciphertext],
pub fn encrypt_data(msg: &str, password: &str) -> Result<String> {
    log::trace!("Encryption attempted");

    let key_bytes = derive_key(password);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce_bytes.into(), msg.as_bytes())
        .map_err(|e| {
            log::error!("failed to encrypt value: {}", e);
            CryptoError::Internal
        })?;

    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    log::trace!("Encryption successfull");
    Ok(format!(
        "{}{}",
        ENCRYPTION_PREFIX,
        general_purpose::STANDARD.encode(&combined)
    ))
}
