use create::CreateSubcommand;

use crate::cli::commands::template::TemplateError;

use super::manager::TemplateManager;

pub mod create;

#[derive(clap::Subcommand, Debug)]
pub enum TemplateSubcommands {
    Create(CreateSubcommand),
    Rename,
    Edit,
    Delete,
}

impl TemplateSubcommands {
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        match self {
            Self::Create(cmd) => cmd.execute(manager)?,
            Self::Edit => println!("execute edit"),
            Self::Rename => println!("execute rename"),
            Self::Delete => println!("execute delete"),
        };

        Ok(())
    }
}
