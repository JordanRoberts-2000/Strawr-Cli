use std::path::PathBuf;

use crate::services::img::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        self.atomic_save(path)?;

        Ok(())
    }

    fn atomic_save(&self, output: &PathBuf) -> Result<()> {
        let file_name = output
            .file_name()
            .ok_or_else(|| ImgError::MissingFileName(output.clone()))?;
        let temp_path = output.with_file_name(format!(".{}", file_name.to_string_lossy()));

        self.img.save(&temp_path).map_err(|e| ImgError::Save {
            source: e,
            output: temp_path.clone(),
        })?;

        std::fs::rename(&temp_path, &output).map_err(|e| ImgError::Io {
            source: e,
            context: format!("failed to rename '{:?}' to '{:?}'", temp_path, output),
        })?;

        Ok(())
    }
}
