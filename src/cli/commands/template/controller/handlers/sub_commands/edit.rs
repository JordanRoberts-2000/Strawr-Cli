use crate::{
    template::{sub_commands::EditSubcommand, TemplateController, TemplateError},
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_edit_subcommand(
        &self,
        args: &EditSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);
        if let Some(input) = &args.template {
            return self
                .resolve_template(input, &args.variant)?
                .edit_template(editor);
        }
        Ok(())
    }
}
