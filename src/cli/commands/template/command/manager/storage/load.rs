use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

use std::fs;

impl TemplateManager {
    pub fn load_templates(&mut self) -> Result<(), TemplateError> {
        let entries =
            fs::read_dir(&self.folder_path).map_err(TemplateError::FailedToReadTemplateDir)?;

        for entry in entries {
            let entry = entry.map_err(TemplateError::FailedToReadTemplateDir)?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    self.templates.push(name.to_string());
                }
            }
        }

        Ok(())
    }
}
