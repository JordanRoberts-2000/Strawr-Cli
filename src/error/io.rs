use std::{io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("Failed to create directory at `{1}`: {0}")]
    CreateDir(#[source] io::Error, PathBuf),

    #[error("Failed to read directory at `{1}`: {0}")]
    ReadDir(#[source] io::Error, PathBuf),

    #[error("Failed to retrieve the current working directory: {0}")]
    GetCurrentDir(std::io::Error),

    #[error("Failed to copy directory contents from `{1}` to `{2}`: {0}")]
    CopyDirContents(#[source] io::Error, PathBuf, PathBuf),

    #[error("Failed to write to file at `{1}`: {0}")]
    WriteFile(#[source] io::Error, PathBuf),

    #[error("Failed to read file at `{1}`: {0}")]
    ReadFile(#[source] io::Error, PathBuf),

    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("path is not a file: {0}")]
    NotAFile(PathBuf),
}
