use std::path::Path;

use crate::error::IoError;

pub fn delete(path: impl AsRef<Path>) -> Result<(), IoError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(IoError::PathNotFound(path.to_path_buf()));
    }

    let md = std::fs::metadata(path).map_err(|e| IoError::Stat(e, path.to_path_buf()))?;

    if md.is_dir() {
        std::fs::remove_dir_all(path).map_err(|e| IoError::DeleteDir(e, path.to_path_buf()))?;
    } else {
        std::fs::remove_file(path).map_err(|e| IoError::DeleteFile(e, path.to_path_buf()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn delete_file_removes_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.touch().unwrap();

        delete(file.path()).unwrap();
        file.assert(predicate::path::missing());
    }

    #[test]
    fn delete_empty_dir_removes_directory() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.child("empty");
        dir.create_dir_all().unwrap();

        delete(dir.path()).unwrap();
        dir.assert(predicate::path::missing());
    }

    #[test]
    fn delete_nested_dir_removes_all_contents() {
        let temp = assert_fs::TempDir::new().unwrap();
        let root = temp.child("root");
        root.create_dir_all().unwrap();
        let nested = root.child("nested");
        nested.create_dir_all().unwrap();
        let file = nested.child("a.txt");
        file.touch().unwrap();

        // Should succeed and remove the entire tree
        delete(root.path()).unwrap();
        root.assert(predicate::path::missing());
        // Confirm nested and file are also gone
        nested.assert(predicate::path::missing());
        file.assert(predicate::path::missing());
    }

    #[test]
    fn errors_when_path_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("no_such");

        // Path does not exist, expect PathNotFound
        let err = delete(missing.path()).unwrap_err();
        match err {
            IoError::PathNotFound(p) => {
                assert_eq!(p, missing.path().to_path_buf());
            }
            other => panic!("expected PathNotFound, got {:?}", other),
        }
    }
}
