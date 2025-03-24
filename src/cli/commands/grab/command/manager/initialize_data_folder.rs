use std::fs;

use serde_json::json;

use crate::error::{Error, Result};
use crate::state::AppContext;

use super::GrabManager;

impl GrabManager {
    pub fn initialize_data_folder(&mut self, ctx: &AppContext) -> Result<()> {
        log::trace!("Attempting to initialize data folder");

        let grab_folder_path = ctx.storage_dir.join("grab");
        self.json_file_path = grab_folder_path.join("data.json");
        self.list_file_path = grab_folder_path.join("keys.list");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path).map_err(|e| Error::Io {
                source: e,
                context: format!("failed to create {:?}", grab_folder_path),
            })?;
            log::info!("Grab folder created");
        }

        if !self.json_file_path.exists() {
            let default_data = json!({});
            fs::write(&self.json_file_path, default_data.to_string()).map_err(|e| Error::Io {
                source: e,
                context: format!("failed to initialize {:?}", self.json_file_path),
            })?;
            log::info!("grab/data.json created");
        }

        if !self.list_file_path.exists() {
            fs::write(&self.list_file_path, "").map_err(|e| Error::Io {
                source: e,
                context: format!("failed to initialize {:?}", self.list_file_path),
            })?;
            log::info!("grab/keys.list created");
        }

        log::trace!("Data folder initialized");
        Ok(())
    }
}
