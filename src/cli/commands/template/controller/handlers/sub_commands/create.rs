use crate::template::{
    context::CreateSubcommandContext, sub_commands::CreateSubcommand, TemplateController,
    TemplateError,
};

impl TemplateController {
    pub fn handle_create_subcommand(
        &self,
        args: &CreateSubcommand,
        ctx: &CreateSubcommandContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
