use arboard::Clipboard;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use crate::state::AppContext;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    pub key: String,

    #[arg(short, long)]
    pub delete: bool,

    #[arg(short, long)]
    pub value: Option<String>,
}

impl GrabCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<(), Box<dyn std::error::Error>> {
        ctx.debug_log("Grab Command Called");

        let (json_file_path, list_file_path) = self.initialize_data_folder(&ctx)?;

        let json_data = fs::read_to_string(&json_file_path)?;
        let mut grab_data_map: HashMap<String, String> = serde_json::from_str(&json_data)?;

        if self.delete {
            if grab_data_map.remove(&self.key).is_some() {
                ctx.debug_log(&format!("Deleting key '{}'", self.key));

                // Overwrite JSON file with the updated map
                let updated_json = serde_json::to_string_pretty(&grab_data_map)?;
                fs::write(&json_file_path, updated_json)?;
                ctx.debug_log(&format!("Key '{}' removed from data.json", self.key));

                // Remove key from keys.list
                let file = File::open(&list_file_path)?;
                let reader = BufReader::new(file);
                let keys: Vec<String> = reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .filter(|line| line.trim() != self.key)
                    .collect();

                let file = File::create(&list_file_path)?;
                let mut writer = BufWriter::new(file);
                for key in keys {
                    writeln!(writer, "{}", key)?;
                }
                ctx.debug_log(&format!("Key '{}' removed from keys.list", self.key));
            } else {
                println!("Key '{}' doesn't exist", self.key)
            }

            return Ok(());
        }

        if let Some(value) = &self.value {
            if let Some(existing_entry) = grab_data_map.get_mut(&self.key) {
                ctx.debug_log(&format!(
                    "key '{}' value has been replaced with '{}'",
                    self.key, value
                ));
                *existing_entry = value.clone();
            } else {
                ctx.debug_log(
                    format!("New entry added: key '{}', value '{}'", self.key, value).as_str(),
                );
                grab_data_map.insert(self.key.clone(), value.clone());

                let mut keys_list = OpenOptions::new().append(true).open(list_file_path)?;

                writeln!(keys_list, "{}", self.key)?;
                ctx.debug_log("Updated keys.list file");
            }

            let updated_json = serde_json::to_string_pretty(&grab_data_map)?;
            fs::write(&json_file_path, updated_json)?;
            ctx.debug_log("Updated data.json file");

            return Ok(());
        }

        if let Some(entry) = grab_data_map.get(&self.key) {
            ctx.debug_log(&format!("Value '{}' copied to clipboard", entry));
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(entry)?;
        } else {
            println!("Key '{}' not found", self.key);
        }

        Ok(())
    }

    pub fn initialize_data_folder(
        &self,
        ctx: &AppContext,
    ) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
        let grab_folder_path = ctx.config_path.join("grab");
        let data_file_path = grab_folder_path.join("data.json");
        let keys_file_path = grab_folder_path.join("keys.list");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path)?;
            ctx.debug_log("grab folder created");
        }

        if !data_file_path.exists() {
            let default_data = json!({});
            fs::write(&data_file_path, default_data.to_string())?;
            ctx.debug_log("grab/data.json created");
        }

        if !keys_file_path.exists() {
            fs::write(&keys_file_path, "")?;
            ctx.debug_log("grab/keys.list created");
        }

        Ok((data_file_path, keys_file_path))
    }
}
