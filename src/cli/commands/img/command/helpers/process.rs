use std::path::PathBuf;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::Img,
};

impl ImgCommand {
    pub fn process_image(&self, image: &mut Img) -> Result<PathBuf, ImgError> {
        Ok(PathBuf::new())
    }
}
