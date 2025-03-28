use crate::services::img::ImgError;
use crate::services::img::{core::ImgSrc, error::Result, Img};
use crate::utils;

impl Img {
    pub fn save(&self) -> Result<()> {
        match &self.src {
            ImgSrc::Local { path, target } => {
                self.save_to(&target)?;
                if path != target {
                    utils::trash(&path).map_err(|e| ImgError::Io {
                        context: format!("failed to delete '{:?}'", path),
                        source: e,
                    })?;
                }
            }
            ImgSrc::Remote { target, .. } => {
                self.save_to(&target)?;
            }
        }

        Ok(())
    }
}
