use super::Img;
use crate::services::img::error::{ImgError, Result};
use std::{fs, path::PathBuf};

impl Img {
    pub fn save(&self) -> Result<()> {
        let mut output_path = self.path.clone();

        if let Some(extention) = self.extention() {
            output_path.set_extension(extention);
        }

        if output_path != self.path {
            if let Err(trash_err) = trash::delete(&self.path) {
                fs::remove_file(&self.path).map_err(|fs_err| ImgError::Io {
                    context: format!(
                        "failed to delete file '{:?}' (trash error: {:?})",
                        self.path, trash_err
                    ),
                    source: fs_err,
                })?;
            }
        }

        self.img.save(output_path).map_err(|e| ImgError::Save {
            source: e,
            output: self.path.clone(),
        })?;

        Ok(())
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        let mut output_path = path.clone();

        if let Some(extention) = self.extention() {
            output_path.set_extension(extention);
        }

        self.img.save(&output_path).map_err(|e| ImgError::Save {
            source: e,
            output: output_path.clone(),
        })?;

        Ok(())
    }
}
