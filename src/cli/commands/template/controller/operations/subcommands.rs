use crate::{
    template::{TemplateController, TemplateError, TemplateSubcommand},
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
                self.handle_create_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Delete(cmd) => {
                self.handle_delete_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Rename(cmd) => {
                self.handle_rename_subcommand(&cmd, &ctx)?;
            }
            TemplateSubcommand::Edit(cmd) => {
                self.handle_edit_subcommand(&cmd, &ctx)?;
            }
        };

        Ok(())
    }
}
