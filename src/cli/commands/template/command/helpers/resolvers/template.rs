use crate::cli::commands::template::{
    command::{manager::TemplateManager, TemplateInput},
    TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_template(
        &self,
        manager: &TemplateManager,
        template: &TemplateInput,
    ) -> Result<(), TemplateError> {
        let (template, inline_variant) = template;
        let template_path = manager.templates_path.join(template);

        if !template_path.exists() {
            return Err(TemplateError::TemplateNotFound(template.clone()));
        }

        let variant = self.resolve_variant(&manager, inline_variant, template, &template_path)?;

        manager.inject_template_files(template, &variant)?;
        Ok(())
    }
}
