use crate::{
    template::{context::*, TemplateController, TemplateError, TemplateSubcommand},
    CliContext,
};

impl TemplateController {
    pub fn handle_subcommands(
        &self,
        subcommands: &TemplateSubcommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        match subcommands {
            TemplateSubcommand::Create(cmd) => {
                let ctx = CreateSubcommandContext::new(&cmd, &ctx);
                self.execute_create_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Delete(cmd) => {
                let ctx = DeleteSubcommandContext::new(&ctx);
                self.execute_delete_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Rename(cmd) => {
                let ctx = RenameSubcommandContext::new(&ctx);
                self.execute_rename_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Edit(cmd) => {
                let ctx = EditSubcommandContext::new(&cmd, &ctx);
                self.execute_edit_subcommand(&cmd, &ctx)?;
            }
        };

        Ok(())
    }
}
