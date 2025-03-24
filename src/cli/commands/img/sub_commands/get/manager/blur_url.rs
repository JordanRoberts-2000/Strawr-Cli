use crate::{cli::commands::img::ImgError, error::Result, utils::to_clipboard};

use super::GetManager;

impl GetManager {
    pub fn handle_blur_url(&mut self) -> Result<()> {
        let data_url = self
            .img
            .max_size(self.config.placeholder_size)
            .blur(self.config.placeholder_blur_intensity)
            .data_url()
            .map_err(ImgError::ImgFailed)?;

        to_clipboard(&data_url)?;
        println!("Blurred data URL successfully copied to clipboard");

        Ok(())
    }
}
