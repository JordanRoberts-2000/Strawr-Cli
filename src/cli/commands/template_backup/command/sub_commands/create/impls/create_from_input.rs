use crate::template::{
    sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
    TemplateError, TemplateManager,
};

impl CreateSubcommand {
    pub fn create_from_input(
        &self,
        manager: &TemplateManager,
        ctx: &CreateSubcommandContext,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        match raw_variant {
            Some(v) => {
                let template = manager.new_template(raw_template)?;
                let variant = manager.create_variant(&template, v)?;
                ctx.service.launch_editor(&ctx.editor, &variant.path)?;
            }
            None => {
                let template = manager.create_template(raw_template)?;
                ctx.service
                    .launch_editor(&ctx.editor, &template.default_variant_path)?;
            }
        }

        Ok(())
    }
}
