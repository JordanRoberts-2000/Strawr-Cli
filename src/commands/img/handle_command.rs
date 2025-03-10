use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::commands::img::image_service::ImageService;
use crate::commands::img::utils::valid_image_format::ValidImageFormat;
use crate::error::{Error, Result};
use crate::state::AppContext;

use super::{ImgCommand, ImgSubcommands};

impl ImgCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<()> {
        log::trace!("Img Command Called");

        if let Some(subcommand) = &self.subcommands {
            match subcommand {
                ImgSubcommands::Gen => println!("Handle_gen"),
                ImgSubcommands::Get => println!("Handle_get"),
            }
        }

        let path = self.validate_path()?;
        let settings = self.transform_settings(ctx);

        if path.is_file() {
            // self.handle_file(&path, &settings)?;

            // todo: also check if current format is the same as the transform format
            if settings.format != ValidImageFormat::Original {
                let img = image::open(&path).expect("Couldnt open");

                // Open the output file.
                let output_file = File::create(&PathBuf::from(
                    "/Users/jordanroberts/Documents/dev/Projects/main/rustCli/playground/img/egg",
                ).with_extension(settings.format.extension()?))
                .expect("failed to create file");
                let mut writer = BufWriter::new(output_file);

                // Write the image to the output file in the specified format.
                img.write_to(
                    &mut writer,
                    settings.format.try_into().expect("Conversion failed"),
                )
                .expect("failed to write to file");
            }
        } else if path.is_dir() {
            // self.handle_folder(&path, &settings)?;
        } else {
            // error
        }

        Ok(())
    }
}
