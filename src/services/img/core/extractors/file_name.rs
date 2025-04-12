use crate::services::img::{Img, ImgError};

impl Img {
    pub fn file_name(&self) -> Result<String, ImgError> {
        let file_name = self
            .target_path
            .file_name()
            .ok_or_else(|| ImgError::MissingFileName(self.target_path.clone()))?;

        Ok(file_name.to_string_lossy().to_string())
    }
}
