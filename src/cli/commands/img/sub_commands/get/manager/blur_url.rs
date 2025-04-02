use crate::{cli::commands::img::ImgError, utils};

use super::GetManager;

impl GetManager {
    pub fn handle_blur_url(&mut self) -> Result<(), ImgError> {
        let data_url = self
            .img
            .max_size(self.config.placeholder_size)
            .blur(self.config.placeholder_blur_intensity)
            .data_url()?;

        utils::clipboard(&data_url)?;
        println!("Blurred data URL successfully copied to clipboard");

        Ok(())
    }
}
