use std::path::{Path, PathBuf};

use crate::validation::ValidationError;

pub fn existing_file(path: impl AsRef<Path>) -> Result<PathBuf, ValidationError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()));
    }

    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn ok_when_path_is_existing_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.touch().unwrap();

        file.assert(predicate::path::exists());
        file.assert(predicate::path::is_file());

        let result = existing_file(file.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file.path().to_path_buf());
    }

    #[test]
    fn err_path_not_found_when_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("does_not_exist.txt");

        let err = existing_file(missing.path()).unwrap_err();
        match err {
            ValidationError::PathNotFound(p) => assert_eq!(p, missing.path().to_path_buf()),
            _ => panic!("expected PathNotFound, got {:?}", err),
        }
    }

    #[test]
    fn err_not_a_file_for_directory() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.child("some_dir");
        dir.create_dir_all().unwrap();

        dir.assert(predicate::path::exists());
        dir.assert(predicate::path::is_dir());

        let err = existing_file(dir.path()).unwrap_err();
        match err {
            ValidationError::NotAFile(p) => assert_eq!(p, dir.path().to_path_buf()),
            _ => panic!("expected NotAFile, got {:?}", err),
        }
    }
}
