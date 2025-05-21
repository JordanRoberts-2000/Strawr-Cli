use std::path::Path;

use crate::error::IoError;

pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), IoError> {
    let from = from.as_ref();
    let to = to.as_ref();

    std::fs::rename(from, to).map_err(|e| IoError::Rename(e, from.to_path_buf(), to.to_path_buf()))
}
