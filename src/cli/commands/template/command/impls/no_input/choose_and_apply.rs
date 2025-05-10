use crate::template::{TemplateCommand, TemplateContext, TemplateError, TemplateManager};

impl TemplateCommand {
    pub fn choose_and_apply_template(
        &self,
        ctx: &TemplateContext,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template:")?;

        if manager.has_variants(&template)? {
            let variant = manager.select_all(&template, "Select variant:")?;
            manager.inject(&template, Some(&variant), &ctx.output)?;
        } else {
            manager.inject(&template, None, &ctx.output)?;
        }

        Ok(())
    }
}
