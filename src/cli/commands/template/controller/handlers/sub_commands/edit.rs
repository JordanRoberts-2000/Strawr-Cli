use crate::{
    template::{
        controller::enums::{NoTemplates, TemplateSelect, VariantArgEmpty},
        sub_commands::EditSubcommand,
        TemplateController, TemplateError,
    },
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
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .edit_template(editor);
        }

        self.handle_no_input(&editor)
            .if_no_templates(NoTemplates::Msg("No templates currently exist".to_string()))?
            .select(TemplateSelect::TemplateOrVariant)?
            .edit_template()?;

        Ok(())
    }
}
