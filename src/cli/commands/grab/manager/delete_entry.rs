use crate::error::{Error, ParseError, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

use super::GrabManager;

impl GrabManager {
    pub fn delete_entry(&mut self, key: &String) -> Result<()> {
        log::trace!("Attempting to delete key '{}'", key);

        if self.data_map.remove(key).is_some() {
            // update JSON data
            let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
                Error::Parse(
                    ParseError::Json(e),
                    "Failed to convert new data to json".to_string(),
                )
            })?;
            fs::write(&self.json_file_path, updated_json).map_err(|e| {
                Error::Io(e, format!("Failed to write to '{:?}'", self.json_file_path))
            })?;
            log::debug!("Key '{}' removed from data.json", key);

            // Remove key from keys.list
            let file = File::open(&self.list_file_path)
                .map_err(|e| Error::Io(e, format!("Failed to open '{:?}'", self.list_file_path)))?;
            let reader = BufReader::new(file);
            let keys: Vec<String> = reader
                .lines()
                .filter_map(|line| line.ok())
                .filter(|line| line.trim() != key)
                .collect();

            let file = File::create(&self.list_file_path).map_err(|e| {
                Error::Io(e, format!("Failed to recreate '{:?}'", self.list_file_path))
            })?;
            let mut writer = BufWriter::new(file);
            for key in keys {
                writeln!(writer, "{}", key).map_err(|e| {
                    Error::Io(e, format!("Failed to write to '{:?}'", self.list_file_path))
                })?;
            }

            println!("Key '{}' removed successfully", key);
        } else {
            println!("Key '{}' doesn't exist", key)
        }

        return Ok(());
    }
}
