use crate::{cli::commands::img::ImgError, utils};

use super::GetManager;

impl GetManager {
    pub fn handle_data_url(&mut self) -> Result<(), ImgError> {
        let data_url = self.img.data_url()?;

        utils::clipboard(&data_url)?;
        println!("Data URL successfully copied to clipboard");

        Ok(())
    }
}
