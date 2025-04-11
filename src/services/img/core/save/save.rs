use crate::services::img::ImgError;
use crate::services::img::{core::ImgSrc, error::Result, Img};
use crate::utils;

impl Img {
    pub fn save(&self) -> Result<()> {
        match &self.src {
            ImgSrc::Local { path } => {
                self.save_to(&self.target_path)?;
                if *path != self.target_path {
                    utils::trash(&path).map_err(|e| ImgError::Io {
                        context: format!("failed to delete '{:?}'", path),
                        source: e,
                    })?;
                }
            }
            ImgSrc::Remote { .. } => {
                self.save_to(&self.target_path)?;
            }
        }

        Ok(())
    }
}
