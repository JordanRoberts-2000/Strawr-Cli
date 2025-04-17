use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    error::ParseError,
    services::crypto,
    utils::Keyring,
};
use std::fs;

impl GrabManager {
    pub fn set_entry(
        &mut self,
        key: &String,
        value: &String,
        encrypt: &bool,
    ) -> Result<(), GrabError> {
        log::trace!("attempting to set key '{}' to '{}'", key, value);

        let entry_value = if *encrypt {
            let password = Keyring::Password.retrieve()?;
            crypto::encrypt(value, &password)?
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

        let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
            ParseError::JsonSerialize {
                source: e,
                title: "updated data".to_string(),
            }
        })?;
        fs::write(&self.json_file_path, updated_json).map_err(|e| GrabError::Io {
            source: e,
            context: format!("Failed to write to '{:?}'", self.json_file_path),
        })?;
        log::debug!("Updated data.json file");

        if entry_already_exists {
            println!("'{}' edited successfully", key);
        } else {
            println!("'{}' created successfully", key);
        }

        Ok(())
    }
}
