use crate::{
    template::{
        controller::enums::VariantArgEmpty, sub_commands::RenameSubcommand, TemplateController,
        TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_rename_subcommand(
        &self,
        args: &RenameSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .rename_template();
        }
        Ok(())
    }
}
