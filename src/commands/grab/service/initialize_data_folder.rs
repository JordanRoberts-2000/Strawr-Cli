use std::fs;

use serde_json::json;

use crate::state::AppContext;

use super::GrabService;

impl GrabService {
    pub fn initialize_data_folder(
        &mut self,
        ctx: &AppContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grab_folder_path = ctx.config_path.join("grab");
        self.json_file_path = grab_folder_path.join("data.json");
        self.list_file_path = grab_folder_path.join("keys.list");

        if !grab_folder_path.exists() {
            fs::create_dir(&grab_folder_path)?;
            log::debug!("grab folder created");
        }

        if !self.json_file_path.exists() {
            let default_data = json!({});
            fs::write(&self.json_file_path, default_data.to_string())?;
            log::debug!("grab/data.json created");
        }

        if !self.list_file_path.exists() {
            fs::write(&self.list_file_path, "")?;
            log::debug!("grab/keys.list created");
        }

        Ok(())
    }
}
