use crate::{
    cli::commands::template::{
        command::{manager::TemplateManager, utils::Template},
        TemplateCommand, TemplateError,
    },
    error::IoError,
    utils::fs::is_dir_empty,
};

impl TemplateCommand {
    pub fn handle_no_input(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        log::debug!("No template or subcommand provided, handling empty input");

        let is_empty = is_dir_empty(&manager.templates_path)
            .map_err(|e| IoError::ReadDir(e, manager.templates_path.clone()))?;

        if is_empty {
            self.handle_no_existing_templates(manager)?;
        } else {
            self.handle_existing_templates(manager)?;
        }

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
            let input = manager.ctx.input.text("Enter template name:")?;
            let template = Template::new(&input, &manager.templates_path)?.create(&None)?;
            manager.open_template(&template.path, &None)?;
        }

        Ok(())
    }

    pub fn handle_existing_templates(
        &self,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template:")?;

        if template.has_variants()? {
            let variant = manager.select_variant(&template, "Select variant:")?;
            manager.inject_template_files(&template, Some(&variant), &self.output)?;
        } else {
            manager.inject_template_files(&template, None, &self.output)?;
        }

        Ok(())
    }
}
