use crate::{
    config::constants::{KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    crypto::{encrypt_data, get_or_prompt_keyring},
    error::{Error, ParseError, Result},
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use super::GrabService;

impl GrabService {
    pub fn set_entry(&mut self, key: &String, value: &String, encrypt: &bool) -> Result<()> {
        log::trace!("attempting to set key '{key}' to '{value}'");

        let entry_value = if *encrypt {
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            encrypt_data(value, &password)?
        } else {
            value.clone()
        };

        if let Some(existing_entry) = self.data_map.get_mut(key) {
            log::debug!("key '{key}' value has been replaced with '{entry_value}'",);
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
