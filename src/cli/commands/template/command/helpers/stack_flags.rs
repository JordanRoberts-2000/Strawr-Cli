use crate::cli::commands::template::{
    command::context::TemplateContext, service::TemplateService, TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_stack_flags(
        &self,
        ctx: &TemplateContext,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        for stack in [self.backend.as_ref(), self.frontend.as_ref()] {
            if let Some((raw_template, raw_variant)) = stack {
                let template = service.new_template(raw_template)?;
                let variant = match raw_variant {
                    Some(name) => Some(service.new_variant(&template, name)?),
                    None => None,
                };

                service.inject(&template, variant.as_ref(), &ctx.output)?;
            }
        }

        Ok(())
    }
}
