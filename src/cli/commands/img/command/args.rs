use crate::cli::commands::img::{
    sub_commands::ImgSubcommands,
    utils::enums::{Size, ValidImageFormat},
};

#[derive(clap::Parser, Debug)]
#[command(subcommand_negates_reqs = true)]
pub struct ImgCommand {
    #[command(subcommand)]
    pub subcommands: Option<ImgSubcommands>,

    #[arg(help = "Path to an image file, a folder of images, or an image URL")]
    pub input: Option<String>,

    #[arg(
      short,
      long,
      help = "Output path. If no value is given, a default filename will be generated.",
      num_args = 0..=1
  )]
    pub output: Option<Option<String>>,

    #[arg(short, long, help = "Fomat image file to be converted to")]
    pub format: Option<ValidImageFormat>,

    #[arg(short, long, help = "Configure image quality")]
    pub quality: Option<u8>,

    #[arg(short, long, help = "Configure image quality")]
    pub size: Option<Size>,

    #[arg(long, help = "Image to use lossless compression when possible", action = clap::ArgAction::SetTrue)]
    pub thumbnail: Option<bool>,

    #[arg(long, help = "Image to use lossless compression when possible", action = clap::ArgAction::SetTrue)]
    pub placeholder: Option<bool>,

    #[arg(long, help = "Image to use lossless compression when possible", num_args = 0..=1)]
    pub blur: Option<Option<u8>>,
}
