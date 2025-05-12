use crate::template::{TemplateCommand, TemplateContext, TemplateError, TemplateManager};

impl TemplateCommand {
    pub fn handle_no_input(
        &self,
        ctx: &TemplateContext,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        log::debug!("No template or subcommand provided, handling empty input");

        if manager.has_templates()? {
            return self.choose_and_apply_template(ctx, manager);
        }

        self.handle_creating_initial_template(ctx, manager)
    }
}
