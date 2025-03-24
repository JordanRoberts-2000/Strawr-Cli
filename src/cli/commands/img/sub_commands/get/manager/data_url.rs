use crate::{cli::commands::img::ImgError, error::Result, utils::to_clipboard};

use super::GetManager;

impl GetManager {
    pub fn handle_data_url(&mut self) -> Result<()> {
        let data_url = self.img.data_url().map_err(ImgError::ImgFailed)?;

        to_clipboard(&data_url)?;
        println!("Data URL successfully copied to clipboard");

        Ok(())
    }
}
