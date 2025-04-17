use clap::Subcommand;
use grab::GrabCommand;
use img::ImgCommand;
use temp::TempCommand;

pub mod grab;
pub mod img;
pub mod temp;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Modify image files")]
    Img(ImgCommand),
    #[command(about = "Create temporary environments")]
    Temp(TempCommand),
    #[command(about = "Open saved data folder")]
    Open,
    #[command(about = "Use playground environment")]
    Playground,
    #[command(about = "Set and get encrypted data")]
    Grab(GrabCommand),
    #[command(about = "Create or manipulate templates for your projects")]
    Template,

    #[command(about = "Backup saved data")]
    Backup,
    #[command(about = "Edit configurations")]
    Config,
    #[command(about = "Uninstalls cli tool and deletes its saved data")]
    Uninstall,
}
