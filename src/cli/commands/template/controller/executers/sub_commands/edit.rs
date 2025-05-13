use crate::template::{
    context::EditSubcommandContext, sub_commands::EditSubcommand, TemplateController, TemplateError,
};

impl TemplateController {
    pub fn execute_edit_subcommand(
        &self,
        args: &EditSubcommand,
        ctx: &EditSubcommandContext,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
