use crate::{cli::commands::template::TemplateError, state::AppContext};

use super::{
    args::TemplateCommand,
    helpers::{handle_no_input, inject_template_files, parse_input},
    manager::TemplateManager,
};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        let mut manager = TemplateManager::new(ctx);

        manager.init_storage()?;
        manager.load_templates()?;

        if self.template.is_none() && self.subcommand.is_none() {
            return handle_no_input();
        }

        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(&manager);
        }

        if let Some(template) = &self.template {
            let (template, variant) = parse_input(template, &self.variant)?;
            inject_template_files(&template, &variant)?;
        }

        Ok(())
    }
}
