use crate::template::{TemplateCommand, TemplateContext, TemplateError, TemplateService};

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
