use crate::{cli::commands::template::TemplateError, state::AppContext};

use super::{args::TemplateCommand, helpers::parse_input, manager::TemplateManager};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        let manager = TemplateManager::new(ctx)?;

        if self.template.is_none() && self.subcommand.is_none() {
            return manager.handle_no_input();
        }

        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(&manager);
        }

        if let Some(template) = &self.template {
            let (template, variant) = parse_input(template, &self.variant)?;
            manager.inject_template_files(&template, &variant)?;
        }

        Ok(())
    }
}
