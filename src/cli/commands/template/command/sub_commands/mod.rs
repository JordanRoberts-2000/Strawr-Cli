use create::CreateSubcommand;
use edit::EditSubcommand;

use crate::cli::commands::template::TemplateError;

use super::{execute::TemplateInput, manager::TemplateManager};

pub mod create;
pub mod edit;

#[derive(clap::Subcommand, Debug)]
pub enum TemplateSubcommands {
    Create(CreateSubcommand),
    Rename,
    Edit(EditSubcommand),
    Delete,
}

impl TemplateSubcommands {
    pub fn execute<'a, T: TemplateInput>(
        &self,
        manager: &TemplateManager<'a, T>,
    ) -> Result<(), TemplateError> {
        match self {
            Self::Create(cmd) => cmd.execute(manager)?,
            Self::Edit(cmd) => cmd.execute(manager)?,
            Self::Rename => println!("execute rename"),
            Self::Delete => println!("execute delete"),
        };

        Ok(())
    }
}
