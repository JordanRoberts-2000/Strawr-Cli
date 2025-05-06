use crate::cli::commands::template::{
    command::{context::TemplateContext, service::TemplateService},
    TemplateCommand, TemplateError,
};

pub mod choose_and_apply;
pub mod create_initial;

impl TemplateCommand {
    pub fn handle_no_input(
        &self,
        ctx: &TemplateContext,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        log::debug!("No template or subcommand provided, handling empty input");

        if service.has_templates()? {
            return self.choose_and_apply_template(ctx, service);
        }

        self.handle_creating_initial_template(ctx, service)
    }
}
