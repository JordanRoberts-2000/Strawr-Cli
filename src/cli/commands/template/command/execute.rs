use crate::{cli::commands::template::TemplateError, state::AppContext};

use super::{args::TemplateCommand, helpers::parse_input, manager::TemplateManager};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        log::debug!("Executing Template Command");

        let editor = self.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        let manager = TemplateManager::new(ctx, editor)?;
        log::trace!("TemplateManager initialized");

        if self.template.is_none() && self.subcommand.is_none() {
            log::debug!("No template or subcommand provided, handling empty input");
            return manager.handle_no_input();
        }

        if let Some(subcommand) = &self.subcommand {
            log::debug!("Executing subcommand: {:?}", subcommand);
            return subcommand.execute(&manager);
        }

        if let Some(template) = &self.template {
            let (template, variant) = parse_input(template, &self.variant)?;
            log::trace!("Input parsed - template: '{template}', variant: '{variant:?}'");

            manager.inject_template_files(&template, &variant)?;
            log::trace!("Injected template '{}' into cwd successfully", template);
        }

        Ok(())
    }
}
