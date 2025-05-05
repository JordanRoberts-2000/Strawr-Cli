use crate::cli::commands::template::{
    command::{manager::TemplateManager, utils::Template, TemplateInput},
    TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_template(
        &self,
        manager: &TemplateManager,
        template: &TemplateInput,
    ) -> Result<(), TemplateError> {
        let (template, inline_variant) = template;
        let template = Template::new(&template, &manager.templates_path)?;

        let variant =
            self.resolve_variant(&manager, inline_variant, &template.name, &template.path)?;

        manager.inject_template_files(&template, variant.as_deref(), &self.output)?;
        Ok(())
    }
}
