use clap::Subcommand;
use playground::args::PlaygroundCommands;

pub mod playground;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help(true))]
    // Img(ImgCommands),
    Grab,
    Template,
    Add,
    Font,
    Snippets,
    Project,
    Playground(PlaygroundCommands),
    Uninstall,
    Config,
    Backup,
}
