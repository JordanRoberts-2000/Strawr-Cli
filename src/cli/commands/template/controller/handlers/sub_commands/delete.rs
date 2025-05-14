use crate::template::{
    context::DeleteSubcommandContext, sub_commands::DeleteSubcommand, TemplateController,
    TemplateError,
};

impl TemplateController {
    pub fn handle_delete_subcommand(
        &self,
        args: &DeleteSubcommand,
        ctx: &DeleteSubcommandContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
