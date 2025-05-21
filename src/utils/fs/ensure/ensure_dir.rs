use std::path::{Path, PathBuf};

use crate::error::IoError;

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<PathBuf, IoError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(IoError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(IoError::NotADirectory(path.to_path_buf()));
    }

    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn ok_when_path_is_existing_dir() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.path();

        let res = ensure_dir(dir);
        assert!(res.is_ok());

        let returned = res.unwrap();
        assert_eq!(returned, dir.to_path_buf());
    }

    #[test]
    fn err_path_not_found_when_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("does_not_exist");

        let res = ensure_dir(missing.path());
        assert!(res.is_err());

        match res.unwrap_err() {
            IoError::PathNotFound(p) => assert_eq!(p, missing.path().to_path_buf()),
            other => panic!("expected PathNotFound, got {:?}", other),
        }
    }

    #[test]
    fn err_not_a_directory_for_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.touch().unwrap();

        file.assert(predicate::path::exists());
        file.assert(predicate::path::is_file());

        let res = ensure_dir(file.path());
        assert!(res.is_err());

        match res.unwrap_err() {
            IoError::NotADirectory(p) => assert_eq!(p, file.path().to_path_buf()),
            other => panic!("expected NotADirectory, got {:?}", other),
        }
    }
}
