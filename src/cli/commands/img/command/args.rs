// use super::utils::valid_image_format::ValidImageFormat;

use crate::cli::commands::img::sub_commands::ImgSubcommands;

#[derive(clap::Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct ImgCommand {
    #[command(subcommand)]
    pub subcommands: Option<ImgSubcommands>,

    #[arg(help = "Path to img file or folder containing images", short, long)]
    pub path: Option<String>,

    #[arg(help = "Path to img file or folder (positional argument)", index = 1)]
    pub positional_path: Option<String>,

    #[arg(
        short,
        long,
        help = "Output path, if not provided the input image is overwritten"
    )]
    pub output: Option<String>,

    // #[arg(short, long, help = "Fomat image file to be converted to")]
    // pub format: Option<ValidImageFormat>,
    #[arg(long, help = "Image to use lossless compression when possible", action = clap::ArgAction::SetTrue, conflicts_with = "lossy")]
    pub lossless: bool,

    #[arg(long, help = "Image to use lossy compression when possible", action = clap::ArgAction::SetTrue, conflicts_with = "lossless")]
    pub lossy: bool,
}
