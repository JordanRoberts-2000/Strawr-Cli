use crate::{cli::commands::template::TemplateError, state::AppContext};

#[derive(clap::Subcommand, Debug)]
pub enum TemplateSubcommands {
    Create,
    Rename,
    Edit,
    Delete,
}

impl TemplateSubcommands {
    pub fn execute(&self, _ctx: &AppContext) -> Result<(), TemplateError> {
        match self {
            Self::Create => println!("execute create"),
            Self::Edit => println!("execute edit"),
            Self::Rename => println!("execute rename"),
            Self::Delete => println!("execute delete"),
        };

        Ok(())
    }
}
