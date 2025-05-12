use crate::template::{
    sub_commands::edit::{EditSubcommand, EditSubcommandContext},
    TemplateError, TemplateManager,
};

impl EditSubcommand {
    pub fn edit_from_input(
        &self,
        manager: &TemplateManager,
        ctx: &EditSubcommandContext,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = manager.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = manager.new_variant(&template, &v)?;
                ctx.service.launch_editor(&ctx.editor, &variant.path)?
            }
            None => ctx
                .service
                .launch_editor(&ctx.editor, &template.default_variant_path)?,
        }

        Ok(())
    }
}
