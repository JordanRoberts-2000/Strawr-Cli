use crate::template::{
    sub_commands::rename::{RenameSubcommand, RenameSubcommandContext},
    TemplateError, TemplateManager,
};

impl RenameSubcommand {
    pub fn rename_template_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &RenameSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Template to edit:")?;
        let input = ctx.service.prompt.text("Rename to:")?;

        manager.rename_template(&template, &input)?;
        Ok(())
    }
}
