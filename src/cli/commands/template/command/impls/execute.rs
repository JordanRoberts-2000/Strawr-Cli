use super::super::helpers::{
    template_input::handle_template_input, templates_init::templates_folder_init,
};

use crate::{
    template::{TemplateCommand, TemplateContext, TemplateError, TemplateService},
    CliContext,
};

impl TemplateCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        templates_folder_init(&ctx.storage_dir)?;

        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(ctx);
        }

        let ctx = TemplateContext::new(self, ctx);
        let service = TemplateService::from(&ctx);

        if let Some(template) = &self.template {
            return handle_template_input(&ctx, &service, template);
        }

        if self.backend.is_some() || self.frontend.is_some() {
            return self.handle_stack_flags(&ctx, &service);
        }

        self.handle_no_input(&ctx, &service)?;
        Ok(())
    }
}
