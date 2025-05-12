use crate::template::{
    sub_commands::rename::{RenameSubcommand, RenameSubcommandContext},
    TemplateError, TemplateManager,
};

impl RenameSubcommand {
    pub fn rename_from_input(
        &self,
        manager: &TemplateManager,
        ctx: &RenameSubcommandContext,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = manager.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = manager.new_variant(&template, &v)?;
                let input = ctx.service.prompt().text("Rename to:")?;
                manager.rename_variant(&variant, &input)?;
            }
            None => {
                let input = ctx.service.prompt().text("Rename to:")?;
                manager.rename_template(&template, &input)?;
            }
        };

        Ok(())
    }
}
