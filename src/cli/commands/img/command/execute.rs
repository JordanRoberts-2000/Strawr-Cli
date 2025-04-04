use std::path::PathBuf;

use crate::cli::commands::img::ImgError;
use crate::services::img::Img;
use crate::state::AppContext;

use super::args::ImgCommand;

impl ImgCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        if let Some(subcommand) = &self.subcommands {
            return subcommand.execute(ctx);
        }

        let input_str = self.path.clone().or(self.positional_path.clone()).unwrap();
        let input = PathBuf::from(input_str);

        let mut img = Img::open(&input).unwrap();

        if let Some(blur) = &self.blur {
            let blur_intensity = match blur {
                None => ctx.config.img.blur_intensity,
                Some(val) => *val,
            };
            img.blur(blur_intensity);
        }

        if let Some(output_str) = &self.output {
            let output_path = PathBuf::from(output_str);
            img.save_to(&output_path).expect("egg1");
        } else {
            img.save().expect("egg2");
        }

        Ok(())
    }
}
