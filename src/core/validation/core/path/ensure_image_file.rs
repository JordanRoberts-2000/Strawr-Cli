use crate::validation::ValidationError;
use std::path::{Path, PathBuf};

pub fn existing_image_file(path: impl AsRef<Path>) -> Result<PathBuf, ValidationError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()));
    }

    const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "svg"];

    let ext_ok = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .map_or(false, |e| IMAGE_EXTS.contains(&e.as_str()));

    if !ext_ok {
        return Err(ValidationError::NotAnImageFile(path.to_path_buf()));
    }

    Ok(path.to_path_buf())
}
