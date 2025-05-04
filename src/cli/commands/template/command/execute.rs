use super::{args::TemplateCommand, manager::TemplateManager};
use crate::{cli::commands::template::TemplateError, state::AppContext};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        log::debug!("Executing Template Command");

        let editor = self.editor.as_ref().unwrap_or(&ctx.config.default_editor);
        let manager = TemplateManager::new(ctx, editor)?;

        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(&manager);
        }

        if let Some(template) = &self.template {
            return self.handle_template(&manager, template);
        }

        if self.backend.is_some() || self.frontend.is_some() {
            return self.handle_stack_flags(&manager);
        }

        self.handle_no_input(&manager)?;
        Ok(())
    }
}
