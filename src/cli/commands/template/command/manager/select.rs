use std::fs;

use crate::{
    cli::commands::template::{error::TemplateError, DEFAULT_FOLDER},
    error::io::IoError,
    utils::{fs::subfolders, input},
};

use super::TemplateManager;

impl<'a> TemplateManager<'a> {
    pub fn select_variant(&self, template: &String) -> Result<Option<String>, TemplateError> {
        let template_path = self.templates_path.join(template);

        let mut variants =
            subfolders(&template_path).map_err(|e| IoError::ReadDir(e, template_path.clone()))?;

        if !variants.iter().any(|v| v == DEFAULT_FOLDER) {
            log::warn!(
                "Default variant folder missing, creating '{}'",
                DEFAULT_FOLDER
            );

            let template_default_path = template_path.join(DEFAULT_FOLDER);
            fs::create_dir(&template_default_path)
                .map_err(|e| IoError::CreateDir(e, template_default_path.clone()))?;
            log::info!(
                "Created default folder for '{template}' at '{}'",
                template_default_path.display()
            );
            variants.push(DEFAULT_FOLDER.to_string());
        }

        if variants.len() >= 2 {
            log::debug!("Multiple variants found, prompting user to select");

            let selected_variant = input::select(&variants, "Select variant:").prompt()?;
            log::debug!("User selected variant: '{}'", selected_variant);

            Ok(Some(selected_variant))
        } else {
            log::debug!("Only one variant available, skipping selection");
            Ok(None)
        }
    }

    pub fn select_template(&self) -> Result<String, TemplateError> {
        let templates = subfolders(&self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.clone()))?;

        let selected_template = input::select(&templates, "Select template:").prompt()?;
        log::debug!("User selected template: '{}'", selected_template);

        Ok(selected_template)
    }
}
