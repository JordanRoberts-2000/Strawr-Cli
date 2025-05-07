use crate::cli::commands::template::{
    command::sub_commands::create::{context::CreateSubcommandContext, CreateSubcommand},
    service::TemplateService,
    TemplateError,
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
