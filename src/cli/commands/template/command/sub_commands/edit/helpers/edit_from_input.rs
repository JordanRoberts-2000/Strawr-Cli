use crate::cli::commands::template::{
    command::sub_commands::edit::{context::EditSubcommandContext, EditSubcommand},
    service::TemplateService,
    TemplateError,
};

impl EditSubcommand {
    pub fn edit_from_input(
        &self,
        service: &TemplateService,
        ctx: &EditSubcommandContext,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = service.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = service.new_variant(&template, &v)?;
                service.launch_editor(&ctx.editor, &variant.path)?
            }
            None => service.launch_editor(&ctx.editor, &template.default_variant_path)?,
        }

        Ok(())
    }
}
