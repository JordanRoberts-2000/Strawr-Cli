use std::path::PathBuf;

use crate::utils::fs::{ensure_dir, ensure_file};

pub fn existing_dir(str: &str) -> Result<PathBuf, String> {
    ensure_dir(str).map_err(|e| e.to_string())
}

pub fn existing_file(str: &str) -> Result<PathBuf, String> {
    ensure_file(str).map_err(|e| e.to_string())
}
