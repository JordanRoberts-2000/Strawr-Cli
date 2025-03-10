use crate::error::{Error, Result};
use image::ImageFormat;
use utils::valid_image_format::ValidImageFormat;

pub mod config;
pub mod handle_command;
pub mod image_service;
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

    #[arg(
        short,
        long,
        help = "Output path, if not provided the input image is overwritten"
    )]
    pub output: Option<String>,

    #[arg(short, long, help = "Fomat image file to be converted to")]
    pub format: Option<ValidImageFormat>,
}

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommands {
    Get,
    Gen,
}
