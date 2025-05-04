use crate::cli::commands::template::{
    command::manager::TemplateManager, TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_stack_flags(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        if let Some((template, variant)) = &self.backend {
            self.process_stack_template(manager, template, variant)?;
        }

        if let Some((template, variant)) = &self.frontend {
            self.process_stack_template(manager, template, variant)?;
        }

        Ok(())
    }

    fn process_stack_template(
        &self,
        manager: &TemplateManager,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let template_path = manager.templates_path.join(template);

        if !template_path.exists() {
            return Err(TemplateError::TemplateNotFound(template.to_string()));
        }

        if let Some(v) = variant {
            let variant_path = template_path.join(v);
            if !variant_path.exists() {
                return Err(TemplateError::VariantNotFound {
                    template: template.to_string(),
                    variant: v.clone(),
                });
            }
        }

        manager.inject_template_files(template, variant)?;
        Ok(())
    }
}
