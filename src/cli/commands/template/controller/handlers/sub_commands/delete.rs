use crate::{
    template::{
        controller::enums::{NoTemplates, TemplateSelect, VariantArgEmpty},
        sub_commands::DeleteSubcommand,
        TemplateController, TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_delete_subcommand(
        &self,
        args: &DeleteSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        let editor = &ctx.config.default_editor;

        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .delete_template();
        }

        self.handle_no_input(&editor)
            .if_no_templates(NoTemplates::Msg("No templates currently exist".to_string()))?
            .select(TemplateSelect::TemplateOrVariant)?
            .delete_template()?;

        Ok(())
    }
}
