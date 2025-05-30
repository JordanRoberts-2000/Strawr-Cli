use std::{io, path::PathBuf};

use crate::validation::ValidationError;

#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("Failed to create directory at `{1}`: {0}")]
    CreateDir(#[source] io::Error, PathBuf),

    #[error("Failed to delete directory at `{1}`: {0}")]
    DeleteDir(#[source] io::Error, PathBuf),

    #[error("Failed to delete file at `{1}`: {0}")]
    DeleteFile(#[source] io::Error, PathBuf),

    #[error("Failed to read directory at `{1}`: {0}")]
    ReadDir(#[source] io::Error, PathBuf),

    #[error("Failed to rename directory from `{1}` to `{2}`: {0}")]
    Rename(#[source] io::Error, PathBuf, PathBuf),

    #[error("Failed to retrieve the current working directory: {0}")]
    GetCurrentDir(std::io::Error),

    #[error("Failed to copy directory contents from `{1}` to `{2}`: {0}")]
    Copy(#[source] io::Error, PathBuf, PathBuf),

    #[error("Failed to write to file at `{1}`: {0}")]
    WriteFile(#[source] io::Error, PathBuf),

    #[error("Failed to read file at `{1}`: {0}")]
    ReadFile(#[source] io::Error, PathBuf),

    #[error("Failed to stat path `{1}`: {0}")]
    Stat(#[source] io::Error, PathBuf),

    #[error(transparent)]
    Validation(#[from] ValidationError),
}
