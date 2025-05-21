use std::path::Path;

use crate::error::IoError;

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<(), IoError> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).map_err(|e| IoError::CreateDir(e, path.to_path_buf()))
}
