use clap::Subcommand;
use open::OpenCommand;
use playground::args::PlaygroundCommand;
use uninstall::UninstallCommand;

pub mod open;
pub mod playground;
pub mod uninstall;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Create or manipulate additions for your current projects")]
    Add,
    #[command(about = "Backup saved data")]
    Backup,
    #[command(about = "Download google fonts")]
    Font,
    // #[command(arg_required_else_help(true), about = "Modify image files")]
    // Img(ImgCommands),
    #[command(about = "Open saved data folder")]
    Open(OpenCommand),
    #[command(about = "Use playground environment")]
    Playground(PlaygroundCommand),
    #[command(about = "Set and get encrypted data")]
    Grab,
    #[command(about = "Create or manipulate code snippets")]
    Snippets,
    #[command(about = "Create or manipulate templates for your projects")]
    Template,
    #[command(about = "Uninstalls cli tool and deletes its saved data")]
    Uninstall(UninstallCommand),
}
