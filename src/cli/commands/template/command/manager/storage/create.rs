use std::fs;

use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

impl TemplateManager {
    pub fn create_template(
        &self,
        template: &String,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let mut path = self.folder_path.join(template);

        match variant {
            Some(v) => path = path.join(v),
            None => path = path.join("default"),
        };

        fs::create_dir_all(&path).map_err(|e| TemplateError::Io {
            context: format!("failed to create template folder '{path:?}'"),
            source: e,
        })?;

        Ok(())
    }
}
