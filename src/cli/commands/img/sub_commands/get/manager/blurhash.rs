use crate::{cli::commands::img::ImgError, utils};

use super::GetManager;

impl GetManager {
    pub fn handle_blurhash(&mut self) -> Result<(), ImgError> {
        let hash = self.img.max_size(64).blurhash()?;

        utils::clipboard(&hash)?;
        println!("Copied hash to clipboard");
        Ok(())
    }
}
