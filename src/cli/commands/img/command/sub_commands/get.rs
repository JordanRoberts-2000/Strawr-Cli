use std::path::PathBuf;

use crate::{commands::img::ImgError, utils::validation::adaptors::clap::validate, CliContext};

#[derive(clap::Parser, Debug)]
#[command(group(
  clap::ArgGroup::new("output")
      .args(&["data_url", "blur_data_url", "hash", "color", "alt"])
      .multiple(false)
))]
pub struct GetSubcommmand {
    #[arg(help = "Path to img file or folder (positional argument)", value_parser = validate::existing_image_file)]
    pub path: PathBuf,

    #[arg(
        help = "Retrieve the data URL representation of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub data_url: bool,

    #[arg(
        help = "Retrieve the blur data URL representation of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub blur_data_url: bool,

    #[arg(
        help = "Retrieve the blurhash string of the image",
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub hash: bool,

    #[arg(
        help = "Retrieve the dominant color or color profile of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub color: bool,

    #[arg(
        help = "Generate or retrieve alternative text (alt) for the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub alt: bool,
}

impl GetSubcommmand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgError> {
        println!("GET SUBCOMMAND");
        Ok(())
    }
}
