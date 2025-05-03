use crate::{error::CliError, state::AppContext};

use super::commands::{
    grab::GrabCommand, img::ImgCommand, temp::TempCommand, template::TemplateCommand,
};

pub mod backup;
pub mod config;
pub mod grab;
pub mod img;
pub mod temp;
pub mod template;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[command(about = "Modify image files")]
    Img(ImgCommand),
    #[command(about = "Create temporary environments")]
    Temp(TempCommand),
    #[command(about = "Set and get encrypted data")]
    Grab(GrabCommand),
    #[command(about = "Create or manipulate templates for your projects")]
    Template(TemplateCommand),

    #[command(about = "Backup saved data")]
    Backup,
    #[command(about = "Edit configurations")]
    Config,
    #[command(about = "Uninstalls cli tool and deletes its saved data")]
    Uninstall,
}

impl Command {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), CliError> {
        match self {
            Self::Grab(cmd) => cmd.execute(ctx)?,
            Self::Img(cmd) => cmd.execute(ctx)?,
            Self::Temp(cmd) => cmd.execute(ctx)?,
            Self::Template(cmd) => cmd.execute(ctx)?,

            Self::Backup => todo!(),
            Self::Config => todo!(),
            Self::Uninstall => todo!(),
        }

        Ok(())
    }
}
