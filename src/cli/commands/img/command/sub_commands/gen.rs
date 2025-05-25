use std::path::PathBuf;

use crate::{
    ai::{AiImageModel, Dalle2ImageSize, Dalle3ImageSize, ImageSize},
    commands::img::{utils::dalle_version_parse, ImgError},
    img::Img,
    utils::validation::adaptors::clap::validate,
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
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgError> {
        let from_cli = self.dalle.is_some();

        let mut model = if from_cli {
            // user override
            self.dalle.clone().unwrap()
        } else {
            // config default
            ctx.config.img.gen.prefered_dalle_version.clone()
        };

        // 2) Pick the size (CLI custom > wide/tall flags > config)
        let size = if let Some(ref s) = self.size {
            s.clone()
        } else if self.wide {
            ImageSize::Wide
        } else if self.tall {
            ImageSize::Tall
        } else {
            ctx.config.img.gen.default_img_size.clone()
        };

        // 3) Only auto-fallback *if* we’re using the config default
        if !from_cli {
            model = match model {
                AiImageModel::Dalle3 if Dalle3ImageSize::try_from(&size).is_err() => {
                    // config said 3, but size not supported → switch to 2
                    AiImageModel::Dalle2
                }
                AiImageModel::Dalle2 if Dalle2ImageSize::try_from(&size).is_err() => {
                    // config said 2, but size not supported → switch to 3
                    AiImageModel::Dalle3
                }
                other => other, // includes Custom(_)
            };
        }

        println!("model: {:?}", model);

        // 4) Call the AI service
        let url = ctx.service.init_ai()?.generate_image(
            &self.description,
            model.clone(),
            size.clone(),
        )?;

        // 5) Download & save
        Img::download(&url)?.save_to(&self.output)?;

        Ok(())
    }
}
