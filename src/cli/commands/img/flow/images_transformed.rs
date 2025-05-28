use std::path::PathBuf;

use crate::commands::img::ImgCmdError;

use super::{core::ImagesTransformed, ImgFlow};

impl ImgFlow<ImagesTransformed> {
    pub fn save(self, output: &Option<Option<PathBuf>>) -> Result<(), ImgCmdError> {
        for img in self.images.iter() {
            img.save_to("./playground/refactor")?;
        }

        Ok(())
    }
}
