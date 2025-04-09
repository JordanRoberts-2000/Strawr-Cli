use std::path::PathBuf;

use crate::cli::commands::img::ImgError;
use crate::services::img::Img;
use crate::state::AppContext;

use super::args::ImgCommand;
use super::helpers::input_detection::InputType;

impl ImgCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        if let Some(subcommand) = &self.subcommands {
            return subcommand.execute(ctx);
        }

        let input_type = self.detect_input_type()?;
        log::debug!("Input was type: {:?}", input_type);

        match &input_type {
            InputType::Directory => self.handle_directory()?,
            InputType::File => {
                let mut img = Img::open(&self.input)?;
                let output = self.process_image(&mut img, &ctx)?;
                img.save_to(&output)?;
            }
            InputType::Url => {
                let mut img = Img::download(&self.input)?;
                let output = self.process_image(&mut img, &ctx)?;
                img.save_to(&output)?;
            }
        }

        Ok(())
    }
}
