use crate::cli::commands::template::{
    command::sub_commands::create::{context::CreateSubcommandContext, CreateSubcommand},
    service::TemplateService,
    TemplateError,
};

impl CreateSubcommand {
    pub fn create_from_input(
        &self,
        service: &TemplateService,
        ctx: &CreateSubcommandContext,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        match raw_variant {
            Some(v) => {
                let template = service.new_template(raw_template)?;
                let variant = service.create_variant(&template, v)?;
                service.launch_editor(&ctx.editor, &variant.path)?;
            }
            None => {
                let template = service.create_template(raw_template)?;
                service.launch_editor(&ctx.editor, &template.default_variant_path)?;
            }
        }

        Ok(())
    }
}
