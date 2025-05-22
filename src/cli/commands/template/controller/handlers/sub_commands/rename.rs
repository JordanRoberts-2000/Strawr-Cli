use crate::{
    template::{
        controller::enums::{NoTemplates, TemplateSelect, VariantArgEmpty},
        sub_commands::RenameSubcommand,
        TemplateController, TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_rename_subcommand(
        &self,
        args: &RenameSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        let editor = &ctx.config.default_editor;

        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .rename_template();
        }

        self.handle_no_input(&editor)
            .if_no_templates(NoTemplates::Msg("No templates currently exist".to_string()))?
            .select(TemplateSelect::TemplateOrVariant)?
            .rename_template()?;

        Ok(())
    }
}
