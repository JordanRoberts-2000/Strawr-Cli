use std::{fs, path::Path};

use crate::error::IoError;

use crate::validation::validate;

pub fn dir_entry_count(path: impl AsRef<Path>) -> Result<usize, IoError> {
    let path = validate::existing_dir(path)?;

    let mut entries = fs::read_dir(&path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;
    let mut count = 0;

    while let Some(_) = entries.next() {
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::validation::ValidationError;

    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn returns_zero_for_empty_directory() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.path();

        assert_eq!(dir_entry_count(dir).unwrap(), 0);
    }

    #[test]
    fn counts_files_and_subdirectories() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.child("my_dir");
        dir.create_dir_all().unwrap();

        // Create two files and one subdirectory inside `my_dir`
        let file1 = dir.child("a.txt");
        let file2 = dir.child("b.txt");
        file1.touch().unwrap();
        file2.touch().unwrap();

        let sub = dir.child("nested");
        sub.create_dir_all().unwrap();

        assert_eq!(dir_entry_count(dir.path()).unwrap(), 3);
    }

    #[test]
    fn errors_when_path_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("does_not_exist");

        let err = dir_entry_count(missing.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::PathNotFound(p)) => {
                assert_eq!(p, missing.path().to_path_buf());
            }
            other => panic!("expected Validation::PathNotFound, got {:?}", other),
        }
    }

    #[test]
    fn errors_when_path_is_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.touch().unwrap();

        let err = dir_entry_count(file.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::NotADirectory(p)) => {
                assert_eq!(p, file.path().to_path_buf());
            }
            other => panic!("expected NotADirectory, got {:?}", other),
        }
    }
}
