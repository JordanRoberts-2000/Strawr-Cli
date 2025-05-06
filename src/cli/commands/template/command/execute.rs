use super::{
    args::TemplateCommand, helpers::template_input::handle_template_input, service::TemplateService,
};
use crate::{
    cli::commands::template::{command::context::TemplateContext, TemplateError},
    state::AppContext,
};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(ctx);
        }

        // todo need to create the templates/ folder if it doesnt exist

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
