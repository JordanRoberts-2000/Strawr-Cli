use std::path::PathBuf;

use crate::{cli::commands::img::ImgError, error::Result, services::img::Img, state::AppContext};

use super::args::Get;

impl Get {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let path = PathBuf::from(self.path.clone());
        let mut img = Img::new(&path).map_err(ImgError::ImgFailed)?;

        if self.color {
        } else if self.blur_data_url {
            let data_url = img
                .max_size(ctx.config.img.placeholder_size)
                .blur(ctx.config.img.placeholder_blur_intensity)
                .data_url()
                .map_err(ImgError::ImgFailed)?;
            println!("{data_url}");
        } else if self.alt {
        } else {
        }

        Ok(())
    }
}
