use std::path::PathBuf;

use clap::{value_parser, ArgAction, Parser, Subcommand};

use crate::{
    commands::img::{
        enums::{ImageInput, ImageSize, ValidImageFormat},
        utils::aspect_ratio_parse,
        ImgError,
    }, img::Img, utils::validation::adaptors::clap::validate, CliContext
};

// use super::sub_commands::{get::GetSubcommmand, r#gen::GenSubcommand};

#[derive(Parser, Debug)]
#[command(subcommand_negates_reqs = true)]
pub struct ImgCommand {
    // #[command(subcommand)]
    // pub subcommand: Option<ImgSubcommand>,
    #[arg(help = "Path to an image file, directory of images, or a remote image URL")]
    pub input: Option<ImageInput>,

    #[arg(
      short, long,
      num_args = 0..=1,
      help = "Output file or folder. If omitted entirely, a default filename is generated.",
      value_parser = validate::existing_dir
  )]
    pub output: Option<Option<PathBuf>>,

    #[arg(short, long, help = "Desired output image format")]
    pub format: Option<ValidImageFormat>,

    #[arg(
      short, long,
      value_parser = value_parser!(u8).range(1..=100),
      help = "Compression quality for lossy formats (1-100)"
  )]
    pub quality: Option<u8>,

    #[arg(short, long, help = "Resize image to max size")]
    pub size: Option<ImageSize>,

    #[arg(long, help = "Generate a smaller thumbnail version of the image", action = ArgAction::SetTrue)]
    pub thumbnail: Option<bool>,

    #[arg(long, help = "Produce a low-resolution placeholder (blurred) image", action = ArgAction::SetTrue)]
    pub placeholder: Option<bool>,

    #[arg(
      short, long,   
      value_name = "WIDTH:HEIGHT",
      help = "Crop the image to the given aspect ratio (e.g. 16:9)", 
      value_parser = aspect_ratio_parse
    )]
    pub crop: Option<f32>,

    #[arg(long, help = "Produce a cropped square version of the image", action = ArgAction::SetTrue)]
    pub square: Option<bool>,

    #[arg(
      short, long,  
      value_parser = value_parser!(u8).range(1..=20), 
      help = "Apply a blur effect; optional strength level (e.g. --blur 5)", 
      num_args = 0..=1
    )]
    pub blur: Option<Option<u8>>,

    #[arg(short, long, help = "Rename the output file (without extension)", value_parser = validate::slug)]
    pub rename: Option<String>,

    #[arg(
        long,
        help = "Suffix to append to the output filename (before extension)",
        value_parser = validate::slug
    )]
    pub suffix: Option<String>,

    #[arg(long, help = "Prefix to prepend to the output filename", value_parser = validate::slug)]
    pub prefix: Option<String>,
}

// #[derive(Subcommand, Debug)]
// pub enum ImgSubcommand {
//     Get(GetSubcommmand),
//     Gen(GenSubcommand),
// }

impl ImgCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgError> {
        // let url = ctx.service.init_ai()?.generate_image("a zoo animal in a barn")?;
        // println!("{}", url);
        // let mut img = Img::download(&url)?;
        // let data_url = img.data_url()?;
        // let description = ctx.service.init_ai()?.get_image_description(&data_url)?;
        // println!("{}", description);
        
        // let controller = TemplateController::new(&ctx);
        // controller.handle_command(self, &ctx)

        // if let Some(subcommand) = &args.subcommand {
        //     return self.handle_subcommands(subcommand, &ctx);
        // }

        // if let Some(input) = &self.input {
        //     self.resolve_image_input(input)
        //         .process_images()
        //         .save(self.output)?;
        // }

        Ok(())
    }
}
