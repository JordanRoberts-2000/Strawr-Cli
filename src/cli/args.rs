use clap::Subcommand;

use super::img::args::ImgCommands;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Watcher,
    #[command(arg_required_else_help(true))]
    Img(ImgCommands),
    Tw,
    Template,
    Add,
    Font,
    Snippet,
    Project,
    Ping,
    Playground,
    Uninstall,
    Backup,
}
