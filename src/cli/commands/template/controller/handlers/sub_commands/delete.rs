use crate::{
    template::{sub_commands::DeleteSubcommand, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
    pub fn handle_delete_subcommand(
        &self,
        args: &DeleteSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        if let Some(input) = &args.template {
            return self
                .resolve_template(input, &args.variant)?
                .delete_template();
        }

        Ok(())
    }
}
