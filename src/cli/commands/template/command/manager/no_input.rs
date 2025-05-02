use crate::{
    cli::commands::template::error::TemplateError, error::io::IoError, utils::fs::is_dir_empty,
};

use super::TemplateManager;

impl<'a> TemplateManager<'a> {
    pub fn handle_no_input(&self) -> Result<(), TemplateError> {
        log::debug!("No template or subcommand provided, handling empty input");

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
        if self
            .ctx
            .input
            .confirm("No templates currently exist, would you like to create one?")?
        {
            let template = self.ctx.input.text("Enter template name:")?;

            self.create_template(&template, None)?;
            log::trace!("Created template '{template}' successfully");
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
