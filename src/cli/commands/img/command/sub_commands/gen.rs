use std::path::PathBuf;

use crate::{
    ai::{AiImageModel, Dalle2ImageSize, Dalle3ImageSize, ImageSize},
    commands::img::{utils::dalle_version_parse, ImgCmdError},
    img::Img,
    utils::spinner,
    validation::adaptors::clap::validate,
    CliContext,
};

#[derive(clap::Parser, Debug, Clone)]
#[command(arg_required_else_help = true, group(
    clap::ArgGroup::new("orientation")
        .args(&["wide", "tall", "size"])
        .multiple(false)
))]
pub struct GenSubcommand {
    #[arg(help = "Text description or prompt for image generation", value_parser = validate::not_empty)]
    pub description: String,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        help = "Generate a widescreen image (horizontal orientation)"
    )]
    pub wide: bool,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        help = "Generate a tall image (vertical orientation)"
    )]
    pub tall: bool,

    #[arg(
        short,
        long,
        help = "Specify the DALL-E version to use (must be either '2' or '3')",
        value_parser = dalle_version_parse
    )]
    pub dalle: Option<AiImageModel>,

    #[arg(
        short,
        long,
        help = "Specify a custom image size (overrides tall/wide options)"
    )]
    pub size: Option<ImageSize>,

    #[arg(long, short, default_value = ".", value_parser = validate::existing_dir)]
    pub output: PathBuf,
}

impl GenSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgCmdError> {
        let from_cli = self.dalle.is_some();

        let mut model = if from_cli {
            self.dalle.clone().unwrap()
        } else {
            ctx.config.img.gen.prefered_dalle_version.clone()
        };

        let size = if let Some(ref s) = self.size {
            s.clone()
        } else if self.wide {
            ImageSize::Wide
        } else if self.tall {
            ImageSize::Tall
        } else {
            ctx.config.img.gen.default_img_size.clone()
        };

        if !from_cli {
            model = match model {
                AiImageModel::Dalle3 if Dalle3ImageSize::try_from(&size).is_err() => {
                    AiImageModel::Dalle2
                }
                AiImageModel::Dalle2 if Dalle2ImageSize::try_from(&size).is_err() => {
                    AiImageModel::Dalle3
                }
                other => other,
            };
        }

        spinner("Generating image…", || -> Result<(), ImgCmdError> {
            let url = ctx.service.init_ai()?.generate_image(
                &self.description,
                model.clone(),
                size.clone(),
            )?;

            Img::download(&url)?.save_to(&self.output)?;
            Ok(())
        })?;

        Ok(())
    }
}
