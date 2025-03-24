use crate::{
    cli::commands::img::{config::ColorOutput, ImgError},
    error::Result,
    utils::to_clipboard,
};

use super::GetManager;

impl GetManager {
    pub fn handle_color(&mut self) -> Result<()> {
        match self.config.default_get_color_output {
            ColorOutput::Rgb => {
                let rgb = self.img.color().map_err(ImgError::ImgFailed)?.rgb();
                to_clipboard(&rgb)?;
                println!("Copied Rgb to clipboard");
            }
            ColorOutput::Hex => {
                let hex = self.img.color().map_err(ImgError::ImgFailed)?.hex();
                to_clipboard(&hex)?;
                println!("Copied hex to clipboard");
            }
        }

        Ok(())
    }
}
