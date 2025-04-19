use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

impl TemplateManager {
    pub fn init_storage(&self) -> Result<(), TemplateError> {
        if !self.folder_path.exists() {
            std::fs::create_dir(&self.folder_path).map_err(|e| TemplateError::Io {
                source: e,
                context: format!(
                    "Failed to create templates folder at {:?}",
                    self.folder_path
                ),
            })?;
            log::info!("Created templates folder at {:?}", self.folder_path);
        }

        Ok(())
    }
}
