use strum_macros::VariantNames;

use crate::{template::TemplateError, CliContext};

use super::{
    create::CreateSubcommand, delete::DeleteSubcommand, edit::EditSubcommand,
    rename::RenameSubcommand,
};

#[derive(clap::Subcommand, Debug, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum TemplateSubcommand {
    Create(CreateSubcommand),
    Rename(RenameSubcommand),
    Edit(EditSubcommand),
    Delete(DeleteSubcommand),
}

impl TemplateSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        match self {
            Self::Create(cmd) => cmd.execute(ctx)?,
            Self::Edit(cmd) => cmd.execute(ctx)?,
            Self::Rename(cmd) => cmd.execute(ctx)?,
            Self::Delete(cmd) => cmd.execute(ctx)?,
        };

        Ok(())
    }
}
