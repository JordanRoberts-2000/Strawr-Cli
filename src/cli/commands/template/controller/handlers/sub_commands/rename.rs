use crate::template::{
    context::RenameSubcommandContext, sub_commands::RenameSubcommand, TemplateController,
    TemplateError,
};

impl TemplateController {
    pub fn handle_rename_subcommand(
        &self,
        args: &RenameSubcommand,
        ctx: &RenameSubcommandContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
