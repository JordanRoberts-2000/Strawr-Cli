use crate::{
    config::constants::{ENCRYPTION_PREFIX, KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    error::{Error, Result},
    utils::get_or_prompt_keyring::get_or_prompt_keyring,
};
use arboard::Clipboard;
use base64::{engine::general_purpose, Engine};

use super::GrabService;

impl GrabService {
    pub fn get_entry(&self, val: &String) -> Result<()> {
        log::debug!("Processing value for clipboard: '{}'", val);

        let final_value: String = if val.starts_with(ENCRYPTION_PREFIX) {
            let encoded = &val[ENCRYPTION_PREFIX.len()..];

            let encrypted_data = general_purpose::STANDARD
                .decode(encoded)
                .map_err(|e| Error::Custom(format!("failed to decode encrypted value: {}", e)))?;

            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;

            let decrypted_bytes = simple_crypt::decrypt(&encrypted_data, password.as_bytes())
                .map_err(|e| Error::Custom(format!("failed to decrypt value: {}", e)))?;

            String::from_utf8(decrypted_bytes).map_err(|e| {
                Error::Custom(format!("failed to convert decrypted data to UTF-8: {}", e))
            })?
        } else {
            val.clone()
        };

        let mut clipboard = Clipboard::new()
            .map_err(|e| Error::Custom(format!("Failed to access clipboard: {}", e)))?;
        clipboard
            .set_text(final_value)
            .map_err(|e| Error::Custom(format!("Failed to set text to clipboard: {}", e)))?;

        log::debug!("Value successfully copied to clipboard");
        Ok(())
    }
}
