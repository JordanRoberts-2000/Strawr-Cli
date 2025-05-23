use std::{fs, path::Path};

use crate::{error::IoError, utils::validation::validate};

pub fn dir_empty(path: impl AsRef<Path>) -> Result<bool, IoError> {
    let path = validate::existing_dir(path)?;

    let mut entries = fs::read_dir(&path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;
    Ok(entries.next().is_none())
}

#[cfg(test)]
mod tests {
    use crate::utils::validation::ValidationError;

    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn returns_true_for_empty_directory() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.path();

        let is_empty = dir_empty(dir).unwrap();
        assert!(is_empty, "Expected empty directory to return true");
    }

    #[test]
    fn returns_false_for_directory_with_entries() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.child("populated");
        dir.create_dir_all().unwrap();

        // Create a file and a subdirectory inside
        let file = dir.child("file.txt");
        file.touch().unwrap();
        let subdir = dir.child("nested");
        subdir.create_dir_all().unwrap();

        let is_empty = dir_empty(dir.path()).unwrap();
        assert!(!is_empty, "Expected non-empty directory to return false");
    }

    #[test]
    fn errors_when_path_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("no_such_dir");

        let err = dir_empty(missing.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::PathNotFound(p)) => {
                assert_eq!(p, missing.path().to_path_buf());
            }
            other => panic!("expected PathNotFound, got {:?}", other),
        }
    }

    #[test]
    fn errors_when_path_is_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("just_a_file.txt");
        file.touch().unwrap();

        let err = dir_empty(file.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::NotADirectory(p)) => {
                assert_eq!(p, file.path().to_path_buf());
            }
            other => panic!("expected NotADirectory, got {:?}", other),
        }
    }
}
