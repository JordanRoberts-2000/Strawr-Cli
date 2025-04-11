use std::{fs, path::Path};

use crate::services::img::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn save_to<P: AsRef<Path>>(&self, folder_path: P) -> Result<()> {
        let folder_path = folder_path.as_ref();

        if !folder_path.exists() {
            fs::create_dir_all(folder_path).map_err(|e| ImgError::Io {
                source: e,
                context: format!("failed to create output directory {:?}", folder_path),
            })?;
        }

        let file_name = &self.file_name;
        // check if filename exists, if so add 'filename(x)';
        let full_path = folder_path.join(file_name);

        self.atomic_save(&full_path)?;

        Ok(())
    }

    fn atomic_save(&self, output: &Path) -> Result<()> {
        let file_name = output
            .file_name()
            .ok_or_else(|| ImgError::MissingFileName(output.to_path_buf()))?;
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
