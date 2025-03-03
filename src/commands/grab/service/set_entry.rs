use base64::{engine::general_purpose, Engine};

use crate::{
    config::constants::{ENCRYPTION_PREFIX, KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    error::{Error, ParseError, Result},
    utils::get_or_prompt_keyring::get_or_prompt_keyring,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use super::GrabService;

impl GrabService {
    pub fn set_entry(&mut self, key: &String, value: &String, encrypt: &bool) -> Result<()> {
        let entry_value = if *encrypt {
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            let encrypted_data: Vec<u8> =
                simple_crypt::encrypt(value.as_bytes(), password.as_bytes())
                    .map_err(|e| Error::Custom(format!("failed to encrypt value, {}", e)))?;

            ENCRYPTION_PREFIX.to_string() + &general_purpose::STANDARD.encode(&encrypted_data)
        } else {
            value.clone()
        };

        if let Some(existing_entry) = self.data_map.get_mut(key) {
            log::debug!(
                "key '{}' value has been replaced with '{}'",
                key,
                entry_value
            );
            *existing_entry = entry_value;
        } else {
            log::debug!("New entry added: key '{}', value '{}'", key, entry_value);
            self.data_map.insert(key.clone(), entry_value);

            let mut keys_list = OpenOptions::new()
                .append(true)
                .open(&self.list_file_path)
                .map_err(|e| Error::Io(e, "Failed to open grab/keys.list".to_string()))?;

            writeln!(keys_list, "{}", key)
                .map_err(|e| Error::Io(e, "Failed to write to grab/keys.list".to_string()))?;
            log::debug!("Updated keys.list file");
        }

        let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
            Error::Parse(
                ParseError::Json(e),
                "Failed to convert new data to json".to_string(),
            )
        })?;
        fs::write(&self.json_file_path, updated_json)
            .map_err(|e| Error::Io(e, "Failed to write to grab/data.json".to_string()))?;
        log::debug!("Updated data.json file");

        return Ok(());
    }
}
