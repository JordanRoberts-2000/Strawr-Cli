use create::CreateSubcommand;
use edit::EditSubcommand;

use crate::cli::commands::template::TemplateError;

use super::manager::TemplateManager;

pub mod create;
pub mod delete;
pub mod edit;
pub mod rename;

#[derive(clap::Subcommand, Debug)]
pub enum TemplateSubcommands {
    Create(CreateSubcommand),
    Rename,
    Edit(EditSubcommand),
    Delete,
}

impl TemplateSubcommands {
    pub fn execute(&self) -> Result<(), TemplateError> {
        match self {
            Self::Create(cmd) => todo!(),
            Self::Edit(cmd) => todo!(),
            Self::Rename => println!("execute rename"),
            Self::Delete => println!("execute delete"),
        };

        Ok(())
    }
}
