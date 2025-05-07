use crate::cli::commands::template::{
    command::context::TemplateContext, service::TemplateService, TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn choose_and_apply_template(
        &self,
        ctx: &TemplateContext,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Select a template:")?;

        if service.has_variants(&template)? {
            let variant = service.select_all(&template, "Select variant:")?;
            service.inject(&template, Some(&variant), &ctx.output)?;
        } else {
            service.inject(&template, None, &ctx.output)?;
        }

        Ok(())
    }
}
