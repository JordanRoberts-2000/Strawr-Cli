use crate::cli::commands::template::{
    command::manager::TemplateManager, TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_stack_flags(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        if let Some((backend_name, backend_variant)) = &self.backend {
            manager.inject_template_files(backend_name, backend_variant)?;
        }

        if let Some((frontend_name, frontend_variant)) = &self.frontend {
            manager.inject_template_files(frontend_name, frontend_variant)?;
        }

        Ok(())
    }
}
