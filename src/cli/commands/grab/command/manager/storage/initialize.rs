use std::fs;

use serde_json::json;

use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    error::IoError,
    state::AppContext,
};

impl GrabManager {
    pub fn init_storage(&mut self, ctx: &AppContext) -> Result<(), GrabError> {
        let grab_folder_path = ctx.storage_dir.join("grab");
        self.json_file_path = grab_folder_path.join("data.json");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path)
                .map_err(|e| IoError::CreateDir(e, grab_folder_path.clone()))?;
            log::info!("Created grab folder at {:?}", grab_folder_path);
        }

        if !self.json_file_path.exists() {
            let default_data = json!({});
            fs::write(&self.json_file_path, default_data.to_string())
                .map_err(|e| IoError::WriteFile(e, self.json_file_path.clone()))?;
            log::info!("Created data file at {:?}", self.json_file_path);
        }

        log::trace!("Grab data folder initialized successfully");
        Ok(())
    }
}
