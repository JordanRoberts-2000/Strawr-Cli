use crate::{
    commands::img::{
        enums::{ImageInput, ImageSize, ValidImageFormat},
        ImgError,
    },
    CliContext,
};

// use super::sub_commands::{get::GetSubcommmand, r#gen::GenSubcommand};

#[derive(clap::Parser, Debug)]
#[command(subcommand_negates_reqs = true)]
pub struct ImgCommand {
    // #[command(subcommand)]
    // pub subcommand: Option<ImgSubcommand>,
    #[arg(help = "Path to an image file, directory of images, or a remote image URL")]
    pub input: Option<ImageInput>,

    #[arg(
      short,
      long,
      help = "Output file or folder. If omitted entirely, a default filename is generated.",
      num_args = 0..=1
  )]
    pub output: Option<Option<String>>,

    #[arg(short, long, help = "Desired output image format")]
    pub format: Option<ValidImageFormat>,

    #[arg(short, long, help = "Compression quality for lossy formats (1-100)")]
    pub quality: Option<u8>,

    #[arg(short, long, help = "Resize image to max size")]
    pub size: Option<ImageSize>,

    // todo: add crop option
    #[arg(long, help = "Generate a smaller thumbnail version of the image", action = clap::ArgAction::SetTrue)]
    pub thumbnail: Option<bool>,

    #[arg(long, help = "Produce a low-resolution placeholder (blurred) image", action = clap::ArgAction::SetTrue)]
    pub placeholder: Option<bool>,

    #[arg(long, help = "Apply a blur effect; optional strength level (e.g. --blur 5)", num_args = 0..=1)]
    pub blur: Option<Option<u8>>,

    #[arg(short, long, help = "Rename the output file (without extension)")]
    pub rename: Option<String>,

    #[arg(
        long,
        help = "Suffix to append to the output filename (before extension)"
    )]
    pub suffix: Option<String>,

    #[arg(long, help = "Prefix to prepend to the output filename")]
    pub prefix: Option<String>,
}

// #[derive(clap::Subcommand, Debug)]
// pub enum ImgSubcommand {
//     Get(GetSubcommmand),
//     Gen(GenSubcommand),
// }

impl ImgCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgError> {
        println!("Input: {:?}", self.input);
        Ok(())
    }
}
