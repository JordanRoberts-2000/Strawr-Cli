use std::{fs, path::PathBuf};

use super::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn save(&self) -> Result<()> {
        let mut output_path = self.path.clone();

        if let Some(extention) = self.get_extention() {
            output_path.set_extension(extention);
        }

        if output_path != self.path {
            fs::remove_file(&self.path).map_err(|e| ImgError::Io {
                context: format!("failed to delete file '{:?}'", self.path),
                source: e,
            })?;
        }

        self.img.save(output_path).map_err(|e| ImgError::Save {
            source: e,
            output: self.path.clone(),
        })?;

        Ok(())
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        let mut output_path = path.clone();

        if let Some(extention) = self.get_extention() {
            output_path.set_extension(extention);
        }

        self.img.save(&output_path).map_err(|e| ImgError::Save {
            source: e,
            output: output_path.clone(),
        })?;

        Ok(())
    }
}
