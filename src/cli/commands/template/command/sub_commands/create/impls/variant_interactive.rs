use crate::template::{
    sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
    TemplateError, TemplateManager,
};

impl CreateSubcommand {
    pub fn create_variant_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &CreateSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template to add a variant to:")?;
        let input = ctx.service.prompt.text("Variant title:")?;
        let variant = manager.create_variant(&template, &input)?;
        ctx.service.launch_editor(&ctx.editor, &variant.path)?;

        Ok(())
    }
}
