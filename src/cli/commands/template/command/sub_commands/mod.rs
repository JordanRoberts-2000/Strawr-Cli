use create::CreateSubcommand;
use delete::DeleteSubcommand;
use edit::EditSubcommand;
use rename::RenameSubcommand;
use strum_macros::VariantNames;

use crate::{cli::commands::template::TemplateError, state::AppContext};

pub mod create;
pub mod delete;
pub mod edit;
pub mod rename;

#[derive(clap::Subcommand, Debug, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum TemplateSubcommands {
    Create(CreateSubcommand),
    Rename(RenameSubcommand),
    Edit(EditSubcommand),
    Delete(DeleteSubcommand),
}

impl TemplateSubcommands {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        match self {
            Self::Create(cmd) => cmd.execute(ctx)?,
            Self::Edit(cmd) => cmd.execute()?,
            Self::Rename(cmd) => cmd.execute()?,
            Self::Delete(cmd) => cmd.execute(ctx)?,
        };

        Ok(())
    }
}
