use crate::{
    template::{
        controller::enums::VariantArgEmpty, sub_commands::CreateSubcommand, TemplateController,
        TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_create_subcommand(
        &self,
        args: &CreateSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::PromptText)?
                .create_template(editor);
        }
        Ok(())
    }
}
