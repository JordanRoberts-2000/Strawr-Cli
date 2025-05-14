use crate::{
    template::{sub_commands::CreateSubcommand, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
    pub fn handle_create_subcommand(
        &self,
        args: &CreateSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
