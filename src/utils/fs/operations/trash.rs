use std::path::Path;

use crate::{error::IoError, utils::validation::ValidationError};

pub fn trash(path: impl AsRef<Path>) -> Result<(), IoError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(IoError::Validation(ValidationError::PathNotFound(
            path.to_path_buf(),
        )));
    }

    if let Err(trash_err) = trash::delete(path) {
        log::warn!(
            "Failed to trash '{}'. Falling back to permanent delete. err: {:?}",
            path.display(),
            trash_err
        );

        let md = std::fs::metadata(path).map_err(|e| IoError::Stat(e, path.to_path_buf()))?;

        if md.is_dir() {
            std::fs::remove_dir_all(path).map_err(|e| IoError::DeleteDir(e, path.to_path_buf()))?;
        } else {
            std::fs::remove_file(path).map_err(|e| IoError::DeleteFile(e, path.to_path_buf()))?;
        }
    }

    Ok(())
}
