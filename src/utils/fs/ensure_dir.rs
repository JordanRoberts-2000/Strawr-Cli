use std::path::{Path, PathBuf};

use crate::error::IoError;

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<PathBuf, IoError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(IoError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(IoError::NotADirectory(path.to_path_buf()));
    }

    Ok(path.to_path_buf())
}
