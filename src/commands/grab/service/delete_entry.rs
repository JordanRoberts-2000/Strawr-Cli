use crate::error::{Error, ParseError, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

use super::GrabService;

impl GrabService {
    pub fn delete_entry(&mut self, key: &String) -> Result<()> {
        if self.data_map.remove(key).is_some() {
            log::debug!("Deleting key '{}'", key);

            // Overwrite JSON file with the updated map
            let updated_json = serde_json::to_string_pretty(&self.data_map).map_err(|e| {
                Error::Parse(
                    ParseError::Json(e),
                    "Failed to convert new data to json".to_string(),
                )
            })?;
            fs::write(&self.json_file_path, updated_json)
                .map_err(|e| Error::Io(e, "Failed to write to grab/data.json".to_string()))?;
            log::debug!("Key '{}' removed from data.json", key);

            // Remove key from keys.list
            let file = File::open(&self.list_file_path)
                .map_err(|e| Error::Io(e, "Failed to open grab/keys.list".to_string()))?;
            let reader = BufReader::new(file);
            let keys: Vec<String> = reader
                .lines()
                .filter_map(|line| line.ok())
                .filter(|line| line.trim() != key)
                .collect();

            let file = File::create(&self.list_file_path)
                .map_err(|e| Error::Io(e, "Failed to recraete grab/keys.list".to_string()))?;
            let mut writer = BufWriter::new(file);
            for key in keys {
                writeln!(writer, "{}", key)
                    .map_err(|e| Error::Io(e, "Failed to write to grab/keys.list".to_string()))?;
            }
            log::debug!("Key '{}' removed from keys.list", key);
        } else {
            println!("Key '{}' doesn't exist", key)
        }

        return Ok(());
    }
}
