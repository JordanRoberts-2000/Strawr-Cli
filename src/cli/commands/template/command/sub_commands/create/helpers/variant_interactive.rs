use crate::cli::commands::template::{
    command::sub_commands::create::{context::CreateSubcommandContext, CreateSubcommand},
    service::TemplateService,
    TemplateError,
};

impl CreateSubcommand {
    pub fn create_variant_interactive(
        &self,
        service: &TemplateService,
        ctx: &CreateSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Select a template to add a variant to:")?;
        let input = service.text("Variant title:")?;
        let variant = service.create_variant(&template, &input)?;
        service.launch_editor(&ctx.editor, &variant.path)?;

        Ok(())
    }
}
