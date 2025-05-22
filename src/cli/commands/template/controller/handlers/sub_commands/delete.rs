use crate::{
    template::{
        controller::enums::VariantArgEmpty, sub_commands::DeleteSubcommand, TemplateController,
        TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_delete_subcommand(
        &self,
        args: &DeleteSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .delete_template();
        }

        Ok(())
    }
}
