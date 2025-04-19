use crate::cli::commands::img::ImgError;
use crate::services::img::Img;
use crate::state::AppContext;

use super::args::ImgCommand;
use super::helpers::input_detection::InputType;

impl ImgCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(ctx);
        }

        if let Some(input) = &self.input {
            let input_type = self.detect_input_type(&input)?;
            log::debug!("Input was type: {:?}", input_type);

            match &input_type {
                InputType::Directory => self.handle_directory(&input, ctx)?,
                InputType::File => {
                    let mut img = Img::open(&input)?;
                    match self.process_image(&mut img, ctx)? {
                        Some(output) => img.save_to(output),
                        None => img.save(),
                    }?;
                }
                InputType::Url => {
                    let mut img = Img::download(&input)?;
                    match self.process_image(&mut img, ctx)? {
                        Some(output) => img.save_to(output),
                        None => img.save(),
                    }?;
                }
            }
        }

        Ok(())
    }
}
