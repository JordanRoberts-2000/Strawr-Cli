use clap::Subcommand;

use super::img::args::ImgCommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Watcher,
    #[command(arg_required_else_help(true))]
    Img(ImgCommand),
    Tw,
    Template,
    Add,
    Font,
    Snippet,
    Project,
    Ping,
}
