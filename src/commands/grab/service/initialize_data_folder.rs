use std::fs;

use serde_json::json;

use crate::error::{Error, Result};
use crate::state::AppContext;

use super::GrabService;

impl GrabService {
    pub fn initialize_data_folder(&mut self, ctx: &AppContext) -> Result<()> {
        let grab_folder_path = ctx.storage_dir.join("grab");
        self.json_file_path = grab_folder_path.join("data.json");
        self.list_file_path = grab_folder_path.join("keys.list");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path)
                .map_err(|e| Error::Io(e, "failed to create grab folder".to_string()))?;
            log::debug!("grab folder created");
        }

        if !self.json_file_path.exists() {
            let default_data = json!({});
            fs::write(&self.json_file_path, default_data.to_string())
                .map_err(|e| Error::Io(e, "failed to initialize grab/data.json".to_string()))?;
            log::debug!("grab/data.json created");
        }

        if !self.list_file_path.exists() {
            fs::write(&self.list_file_path, "")
                .map_err(|e| Error::Io(e, "failed to initialize grab/keys.list".to_string()))?;
            log::debug!("grab/keys.list created");
        }

        Ok(())
    }
}
