use crate::{commands::*, CliContext, CliError};

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[command(about = "Modify image files")]
    Img(ImgCommand),
    // #[command(about = "Create temporary environments")]
    // Temp(TempCommand),
    // #[command(about = "Set and get encrypted data")]
    // Grab(GrabCommand),
    #[command(about = "Create or manipulate templates for your projects")]
    Template(TemplateCommand),

    #[command(about = "Backup saved data")]
    Backup,
    #[command(about = "Edit configurations")]
    Config(ConfigCommand),
    #[command(about = "Uninstalls cli tool and deletes its saved data")]
    Uninstall,
}

impl Command {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), CliError> {
        match self {
            // Self::Grab(cmd) => cmd.execute(ctx)?,
            Self::Img(cmd) => cmd.execute(ctx)?,
            // Self::Temp(cmd) => cmd.execute(ctx)?,
            Self::Template(cmd) => cmd.execute(ctx)?,
            Self::Backup => todo!(),
            Self::Config(cmd) => cmd.execute(ctx)?,
            Self::Uninstall => todo!(),
        }

        Ok(())
    }
}
