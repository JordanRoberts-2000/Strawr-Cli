use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

use super::GrabService;

impl GrabService {
    pub fn delete_entry(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        if self.data_map.remove(key).is_some() {
            log::debug!("Deleting key '{}'", key);

            // Overwrite JSON file with the updated map
            let updated_json = serde_json::to_string_pretty(&self.data_map)?;
            fs::write(&self.json_file_path, updated_json)?;
            log::debug!("Key '{}' removed from data.json", key);

            // Remove key from keys.list
            let file = File::open(&self.list_file_path)?;
            let reader = BufReader::new(file);
            let keys: Vec<String> = reader
                .lines()
                .filter_map(|line| line.ok())
                .filter(|line| line.trim() != key)
                .collect();

            let file = File::create(&self.list_file_path)?;
            let mut writer = BufWriter::new(file);
            for key in keys {
                writeln!(writer, "{}", key)?;
            }
            log::debug!("Key '{}' removed from keys.list", key);
        } else {
            println!("Key '{}' doesn't exist", key)
        }

        return Ok(());
    }
}
