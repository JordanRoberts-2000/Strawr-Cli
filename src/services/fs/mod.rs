use std::{fs, path::Path};

use crate::{
    error::IoError,
    utils::fs::{copy_dir_contents, is_dir_empty, subfolders},
};

pub struct CliFsRepository;

pub trait FsRepository:
    CreateDir + DeleteDir + DirEmpty + SubDirs + CopyDirContents + MinEntries
{
}

impl<T> FsRepository for T where
    T: CreateDir + DeleteDir + DirEmpty + SubDirs + CopyDirContents + MinEntries
{
}

pub trait CreateDir {
    fn create_dir_all(&self, path: &Path) -> Result<(), IoError>;
}

pub trait DeleteDir {
    fn delete_dir_all(&self, path: &Path) -> Result<(), IoError>;
}

pub trait CopyDirContents {
    fn copy_dir_contents(&self, path: &Path, output: &Path) -> Result<(), IoError>;
}

pub trait DirEmpty {
    fn dir_empty(&self, path: &Path) -> Result<bool, IoError>;
}

pub trait SubDirs {
    fn sub_dirs(&self, path: &Path) -> Result<Vec<String>, IoError>;
}

pub trait MinEntries {
    fn min_entries(&self, path: &Path, min: usize) -> Result<bool, IoError>;
}

impl CreateDir for CliFsRepository {
    fn create_dir_all(&self, path: &Path) -> Result<(), IoError> {
        fs::create_dir_all(path).map_err(|e| IoError::CreateDir(e, path.to_path_buf()))
    }
}

impl DeleteDir for CliFsRepository {
    fn delete_dir_all(&self, path: &Path) -> Result<(), IoError> {
        fs::remove_dir_all(path).map_err(|e| IoError::DeleteDir(e, path.to_path_buf()))
    }
}

impl CopyDirContents for CliFsRepository {
    fn copy_dir_contents(&self, path: &Path, output: &Path) -> Result<(), IoError> {
        copy_dir_contents(&path, output)
            .map_err(|e| IoError::CopyDirContents(e, path.to_path_buf(), output.to_path_buf()))?;

        Ok(())
    }
}

impl DirEmpty for CliFsRepository {
    fn dir_empty(&self, path: &Path) -> Result<bool, IoError> {
        is_dir_empty(path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))
    }
}

impl SubDirs for CliFsRepository {
    fn sub_dirs(&self, path: &Path) -> Result<Vec<String>, IoError> {
        subfolders(path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))
    }
}

impl MinEntries for CliFsRepository {
    fn min_entries(&self, path: &Path, min: usize) -> Result<bool, IoError> {
        let mut entries =
            fs::read_dir(path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;
        let mut count = 0;

        while let Some(_) = entries.next() {
            count += 1;
            if count >= min {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
