use std::{fs, path::Path};

use crate::error::IoError;

use super::ensure_dir;

pub fn dir_entry_count(path: impl AsRef<Path>) -> Result<usize, IoError> {
    let path = ensure_dir(path)?;

    let mut entries = fs::read_dir(&path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;
    let mut count = 0;

    while let Some(_) = entries.next() {
        count += 1;
    }

    Ok(count)
}
