use std::{fs, path::Path};

use crate::error::IoError;

use crate::utils::fs::ensure_dir;

pub fn sub_dirs(path: impl AsRef<Path>) -> Result<Vec<String>, IoError> {
    let path = ensure_dir(path)?;

    let entries = fs::read_dir(&path).map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;

    let mut folder_names = Vec::new();

    for entry_res in entries {
        let entry = entry_res.map_err(|e| IoError::ReadDir(e, path.to_path_buf()))?;
        let p = entry.path();

        if p.is_dir() {
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                folder_names.push(name.to_string());
            }
        }
    }

    Ok(folder_names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn returns_empty_for_empty_directory() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.path();

        let result = sub_dirs(dir).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn ignores_files_and_only_lists_subdirectories() {
        let temp = assert_fs::TempDir::new().unwrap();
        let dir = temp.child("root");
        dir.create_dir_all().unwrap();

        // create two files
        let f1 = dir.child("file1.txt");
        let f2 = dir.child("file2.log");
        f1.touch().unwrap();
        f2.touch().unwrap();

        // create two subdirectories
        let d1 = dir.child("alpha");
        let d2 = dir.child("beta");
        d1.create_dir_all().unwrap();
        d2.create_dir_all().unwrap();

        let mut subs = sub_dirs(dir.path()).unwrap();
        subs.sort();
        assert_eq!(subs, vec!["alpha".to_string(), "beta".to_string()]);
    }

    #[test]
    fn errors_when_path_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let missing = temp.child("nope_dir");

        let err = sub_dirs(missing.path()).unwrap_err();
        match err {
            IoError::PathNotFound(p) => assert_eq!(p, missing.path().to_path_buf()),
            other => panic!("expected PathNotFound, got {:?}", other),
        }
    }

    #[test]
    fn errors_when_path_is_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.touch().unwrap();

        let err = sub_dirs(file.path()).unwrap_err();
        match err {
            IoError::NotADirectory(p) => assert_eq!(p, file.path().to_path_buf()),
            other => panic!("expected NotADirectory, got {:?}", other),
        }
    }
}
