use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateCommand, TemplateError},
    error::IoError,
    utils::fs::is_dir_empty,
};

impl TemplateCommand {
    pub fn handle_no_input(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        log::debug!("No template or subcommand provided, handling empty input");

        let is_empty = is_dir_empty(&manager.templates_path)
            .map_err(|e| IoError::ReadDir(e, manager.templates_path.clone()))?;

        if is_empty {
            log::debug!("Templates directory is empty");
            self.handle_no_existing_templates(manager)?;
        } else {
            log::debug!("Templates directory has existing templates");
            self.handle_existing_templates(manager)?;
        }

        Ok(())
    }

    pub fn handle_existing_templates(
        &self,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template()?;
        let variant = manager.select_variant(&template)?;

        manager.inject_template_files(&template, &variant)?;
        Ok(())
    }

    pub fn handle_no_existing_templates(
        &self,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        if manager
            .ctx
            .input
            .confirm("No templates currently exist, would you like to create one?")?
        {
            println!("THere");
            let template = manager.ctx.input.text("Enter template name:")?;

            manager.create_template(&template, None)?;
            log::trace!("Created template '{template}' successfully");
        }

        Ok(())
    }
}
