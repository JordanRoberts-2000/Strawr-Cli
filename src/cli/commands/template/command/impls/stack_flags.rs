use crate::template::{TemplateCommand, TemplateContext, TemplateError, TemplateManager};

impl TemplateCommand {
    pub fn handle_stack_flags(
        &self,
        ctx: &TemplateContext,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        for stack in [self.backend.as_ref(), self.frontend.as_ref()] {
            if let Some((raw_template, raw_variant)) = stack {
                let template = manager.new_template(raw_template)?;
                let variant = match raw_variant {
                    Some(name) => Some(manager.new_variant(&template, name)?),
                    None => None,
                };

                manager.inject(&template, variant.as_ref(), &ctx.output)?;
            }
        }

        Ok(())
    }
}
