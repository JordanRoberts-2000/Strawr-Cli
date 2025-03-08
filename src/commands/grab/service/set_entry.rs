use crate::{
    config::constants::{KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    crypto::{encrypt_data, get_or_prompt_keyring},
    error::{Error, ParseError, Result},
};
use natord::compare;
use std::fs;

use super::GrabService;

impl GrabService {
    pub fn set_entry(&mut self, key: &String, value: &String, encrypt: &bool) -> Result<()> {
        log::trace!("attempting to set key '{}' to '{}'", key, value);

        let entry_value = if *encrypt {
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            encrypt_data(value, &password)?
        } else {
            value.clone()
        };

        let mut entry_already_exists = false;

        if let Some(existing_entry) = self.data_map.get_mut(key) {
            entry_already_exists = true;
            *existing_entry = entry_value;
        } else {
            self.data_map.insert(key.clone(), entry_value);
        }

        let mut keys: Vec<String> = self.data_map.keys().cloned().collect();
        keys.sort_by(|a, b| compare(a, b));

        fs::write(&self.list_file_path, keys.join("\n"))
            .map_err(|e| Error::Io(e, format!("Failed to write to '{:?}'", self.list_file_path)))?;
        log::debug!("Updated keys.list file");

        let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
            Error::Parse(
                ParseError::Json(e),
                "Failed to convert new data to json".to_string(),
            )
        })?;
        fs::write(&self.json_file_path, updated_json)
            .map_err(|e| Error::Io(e, format!("Failed to write to '{:?}'", self.json_file_path)))?;
        log::debug!("Updated data.json file");

        if entry_already_exists {
            println!("'{}' edited successfully", key);
        } else {
            println!("'{}' created successfully", key);
        }

        Ok(())
    }
}
