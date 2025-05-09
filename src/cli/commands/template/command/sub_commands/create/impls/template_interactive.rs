use crate::template::{
    sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
    TemplateError, TemplateService,
};

impl CreateSubcommand {
    pub fn create_template_interactive(
        &self,
        service: &TemplateService,
        ctx: &CreateSubcommandContext,
    ) -> Result<(), TemplateError> {
        let input = service.text("New Template title:")?;
        let template = service.create_template(&input)?;
        service.launch_editor(&ctx.editor, &template.default_variant_path)?;

        Ok(())
    }
}
