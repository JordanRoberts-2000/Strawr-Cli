use crate::{error::CliError, state::AppContext, utils::input::UserInput};

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
    pub fn execute(&self, ctx: &AppContext, input: &UserInput) -> Result<(), CliError> {
        match self {
            Command::Grab(cmd) => cmd.execute(ctx, input)?,
            Command::Img(cmd) => cmd.execute(ctx)?,
            Command::Temp(cmd) => cmd.execute(ctx)?,
            Command::Template(cmd) => cmd.execute(ctx, input)?,

            Command::Backup => todo!(),
            Command::Config => todo!(),
            Command::Uninstall => todo!(),
        }

        Ok(())
    }
}
