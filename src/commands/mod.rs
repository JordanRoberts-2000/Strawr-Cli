use clap::Subcommand;
use playground::args::PlaygroundCommands;

pub mod playground;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Create or manipulate additions for your current projects")]
    Add,
    #[command(about = "Backup saved data")]
    Backup,
    #[command(about = "Open or reset configuration file")]
    Config,
    #[command(about = "Download google fonts")]
    Font,
    // #[command(arg_required_else_help(true), about = "Modify image files")]
    // Img(ImgCommands),
    #[command(about = "Open saved data folder")]
    Open,
    #[command(about = "Use playground environment")]
    Playground(PlaygroundCommands),
    #[command(about = "Set and get encrypted data")]
    Grab,
    #[command(about = "Create or manipulate code snippets")]
    Snippets,
    #[command(about = "Create or manipulate templates for your projects")]
    Template,
    #[command(about = "Uninstalls cli tool and deletes its saved data")]
    Uninstall,
}
