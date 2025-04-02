use crate::{
    cli::commands::img::{config::ColorOutput, ImgError},
    utils,
};

use super::GetManager;

impl GetManager {
    pub fn handle_color(&mut self) -> Result<(), ImgError> {
        match self.config.get.default_color_output {
            ColorOutput::Rgb => {
                let rgb = self.img.color()?.rgb();
                utils::clipboard(&rgb)?;
                println!("Copied Rgb to clipboard");
            }
            ColorOutput::Hex => {
                let hex = self.img.color()?.hex();
                utils::clipboard(&hex)?;
                println!("Copied hex to clipboard");
            }
        }

        Ok(())
    }
}
