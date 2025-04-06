use std::fs;

use serde_json::json;

use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    state::AppContext,
};

impl GrabManager {
    pub fn init_storage(&mut self, ctx: &AppContext) -> Result<(), GrabError> {
        let grab_folder_path = ctx.storage_dir.join("grab");
        self.json_file_path = grab_folder_path.join("data.json");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path).map_err(|e| GrabError::Io {
                source: e,
                context: format!("Failed to create grab folder at {:?}", grab_folder_path),
            })?;
            log::info!("Created grab folder at {:?}", grab_folder_path);
        }

        if !self.json_file_path.exists() {
            let default_data = json!({});
            fs::write(&self.json_file_path, default_data.to_string()).map_err(|e| {
                GrabError::Io {
                    source: e,
                    context: format!("Failed to create data file at {:?}", self.json_file_path),
                }
            })?;
            log::info!("Created data file at {:?}", self.json_file_path);
        }

        log::trace!("Grab data folder initialized successfully");
        Ok(())
    }
}
