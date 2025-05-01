use std::fs;

use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    error::{IoError, ParseError},
};

impl<'a> GrabManager<'a> {
    pub fn delete_entry(&mut self, key: &String) -> Result<(), GrabError> {
        log::trace!("Attempting to delete key '{}'", key);

        if self.data_map.remove(key).is_some() {
            // Update JSON data
            let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
                ParseError::JsonSerialize {
                    source: e,
                    title: "updated data".to_string(),
                }
            })?;

            fs::write(&self.json_file_path, updated_json)
                .map_err(|e| IoError::WriteFile(e, self.json_file_path.clone()))?;

            log::debug!("Key '{}' removed from data.json", key);
            println!("Key '{}' removed successfully", key);
        } else {
            return Err(GrabError::KeyNotFound(key.clone()));
        }

        Ok(())
    }
}
