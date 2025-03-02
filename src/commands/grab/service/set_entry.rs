use crate::{
    config::constants::{KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
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
        if *encrypt {
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            println!("PASSWORD AHHH: {}", password);
        }

        if let Some(existing_entry) = self.data_map.get_mut(key) {
            log::debug!("key '{}' value has been replaced with '{}'", key, value);
            *existing_entry = value.clone();
        } else {
            log::debug!("New entry added: key '{}', value '{}'", key, value);
            self.data_map.insert(key.clone(), value.clone());

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
