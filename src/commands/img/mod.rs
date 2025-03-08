pub mod handle_command;
pub mod utils;

#[derive(clap::Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct ImgCommand {
    #[command(subcommand)]
    pub subcommands: Option<ImgSubcommands>,

    #[arg(help = "Path to img file or folder containing images", short, long)]
    pub path: Option<String>,

    #[arg(help = "Path to img file or folder (positional argument)", index = 1)]
    pub positional_path: Option<String>,
    // #[arg(short, long, conflicts_with = "flag_b")]
    // pub flag_a: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommands {
    Get,
    Gen,
}
