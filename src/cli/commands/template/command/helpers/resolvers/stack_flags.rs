use crate::cli::commands::template::{
    command::{manager::TemplateManager, utils::Template},
    TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_stack_flags(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        if let Some((template, variant)) = &self.backend {
            let template = Template::new(&template, &manager.templates_path)?;
            manager.inject_template_files(&template, variant.as_deref(), &self.output)?;
        }

        if let Some((template, variant)) = &self.frontend {
            let template = Template::new(&template, &manager.templates_path)?;
            manager.inject_template_files(&template, variant.as_deref(), &self.output)?;
        }

        Ok(())
    }
}
