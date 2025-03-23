use crate::cli::commands::img::{sub_commands::ImgSubcommands, utils::formats::ValidImageFormat};

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

    #[arg(short, long, help = "Fomat image file to be converted to")]
    pub format: Option<ValidImageFormat>,

    #[arg(short, long, help = "Configure image quality")]
    pub quality: Option<u8>,

    #[arg(long, help = "Image to use lossless compression when possible", action = clap::ArgAction::SetTrue)]
    pub thumbnail: Option<bool>,

    #[arg(long, help = "Image to use lossless compression when possible", num_args = 0..=1)]
    pub blur: Option<Option<u8>>,
}
