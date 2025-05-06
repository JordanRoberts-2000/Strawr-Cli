use crate::cli::commands::template::{
    command::{context::TemplateContext, service::TemplateService},
    TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_creating_initial_template(
        &self,
        ctx: &TemplateContext,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let msg = "No templates currently exist, would you like to create one?";

        if service.confirm(msg)? {
            let input = service.text("Enter template name:")?;
            let template = service.create_template(&input)?;
            service.open_template(&template, &ctx.editor)?;
        }

        Ok(())
    }
}
