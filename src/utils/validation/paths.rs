use std::path::PathBuf;

use crate::error::IoError;

pub fn existing_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(IoError::PathNotFound(path.clone()).to_string());
    }

    if !path.is_dir() {
        return Err(IoError::NotADirectory(path.clone()).to_string());
    }

    Ok(path)
}

pub fn existing_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(IoError::PathNotFound(path.clone()).to_string());
    }

    if !path.is_file() {
        return Err(IoError::NotAFile(path.clone()).to_string());
    }

    Ok(path)
}
