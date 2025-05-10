use crate::template::{
    sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
    TemplateError, TemplateManager,
};

impl CreateSubcommand {
    pub fn create_template_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &CreateSubcommandContext,
    ) -> Result<(), TemplateError> {
        let input = ctx.service.prompt.text("New Template title:")?;
        let template = manager.create_template(&input)?;
        ctx.service
            .launch_editor(&ctx.editor, &template.default_variant_path)?;

        Ok(())
    }
}
