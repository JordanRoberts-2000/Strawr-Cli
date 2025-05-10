use crate::template::{
    sub_commands::rename::{RenameSubcommand, RenameSubcommandContext},
    TemplateError, TemplateManager,
};

impl RenameSubcommand {
    pub fn rename_variant_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &RenameSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template to see variants:")?;
        let variant = manager.select_variant(&template, "Select variant to edit:")?;
        let input = ctx.service.prompt.text("Rename to:")?;

        manager.rename_variant(&variant, &input)?;
        Ok(())
    }
}
