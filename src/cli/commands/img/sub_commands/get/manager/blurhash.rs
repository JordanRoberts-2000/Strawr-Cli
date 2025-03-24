use crate::{cli::commands::img::ImgError, error::Result, utils::to_clipboard};

use super::GetManager;

impl GetManager {
    pub fn handle_blurhash(&mut self) -> Result<()> {
        let hash = self
            .img
            .max_size(64)
            .blurhash()
            .map_err(ImgError::ImgFailed)?;

        to_clipboard(&hash)?;
        println!("Copied hash to clipboard");
        Ok(())
    }
}
