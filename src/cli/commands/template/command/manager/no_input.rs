use crate::{
    cli::commands::template::{
        command::helpers::{prompt_create_initial_template, prompt_template_name},
        error::TemplateError,
    },
    error::io::IoError,
    utils::fs::is_dir_empty,
};

use super::TemplateManager;

impl<'a> TemplateManager<'a> {
    pub fn handle_no_input(&self) -> Result<(), TemplateError> {
        let is_empty = is_dir_empty(&self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.clone()))?;

        if is_empty {
            log::debug!("Templates directory is empty");
            self.handle_no_templates()
        } else {
            log::debug!("Templates directory has existing templates");
            self.handle_templates()
        }
    }

    pub fn handle_no_templates(&self) -> Result<(), TemplateError> {
        if prompt_create_initial_template()? {
            match prompt_template_name()? {
                Some(title) => {
                    self.create_template(&title, None)?;
                    log::trace!("Created template '{title}' successfully");
                }
                None => log::info!("Template creation canceled at name input"),
            };
        }

        Ok(())
    }

    pub fn handle_templates(&self) -> Result<(), TemplateError> {
        let template = self.select_template()?;
        let variant = self.select_variant(&template)?;

        self.inject_template_files(&template, &variant)?;

        Ok(())
    }
}
