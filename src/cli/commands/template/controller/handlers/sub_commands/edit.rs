use crate::{
    template::{sub_commands::EditSubcommand, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
    pub fn handle_edit_subcommand(
        &self,
        args: &EditSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
