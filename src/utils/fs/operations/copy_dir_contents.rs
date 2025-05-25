use std::{fs, path::Path};

use crate::error::IoError;

use crate::validation::validate;

pub fn copy_dir_contents(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), IoError> {
    let src = validate::existing_dir(src)?;
    let dst = validate::existing_dir(dst)?;

    let entries = fs::read_dir(&src).map_err(|e| IoError::ReadDir(e, src.to_path_buf()))?;

    for entry_res in entries {
        let entry = entry_res.map_err(|e| IoError::ReadDir(e, src.to_path_buf()))?;
        let entry_path = entry.path();

        let file_name = match entry_path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => {
                log::warn!("Skipping non-UTF8 filename: '{}'", entry_path.display());
                continue;
            }
        };

        let dst_path = dst.join(file_name);

        if entry_path.is_dir() {
            fs::create_dir_all(&dst_path).map_err(|e| IoError::CreateDir(e, dst_path.clone()))?;
            copy_dir_contents(&entry_path, &dst_path)?;
        } else {
            fs::copy(&entry_path, &dst_path)
                .map_err(|e| IoError::Copy(e, entry_path.clone(), dst_path.clone()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::validation::ValidationError;

    use super::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn copies_empty_directory_without_error() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("src");
        let dst = temp.child("dst");
        src.create_dir_all().unwrap();
        dst.create_dir_all().unwrap();

        // Should succeed and leave dst empty
        copy_dir_contents(src.path(), dst.path()).unwrap();
        let entries: Vec<_> = fs::read_dir(dst.path()).unwrap().collect();
        assert!(entries.is_empty(), "dst directory should still be empty");
    }

    #[test]
    fn copies_files_and_subdirectories() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("src");
        let dst = temp.child("dst");
        src.create_dir_all().unwrap();
        dst.create_dir_all().unwrap();

        // Set up a file and a nested directory in src
        let f1 = src.child("hello.txt");
        f1.write_str("world").unwrap();
        let nested = src.child("nested");
        nested.create_dir_all().unwrap();
        let f2 = nested.child("foo.txt");
        f2.write_str("bar").unwrap();

        // Perform the copy
        copy_dir_contents(src.path(), dst.path()).unwrap();

        // Assert top-level file copied
        let copied = dst.child("hello.txt");
        copied.assert(predicate::path::exists());
        let content = fs::read_to_string(copied.path()).unwrap();
        assert_eq!(content, "world");

        // Assert nested directory and file copied
        let nested_dst = dst.child("nested");
        nested_dst.assert(predicate::path::exists());
        nested_dst.assert(predicate::path::is_dir());
        let copied_nested = nested_dst.child("foo.txt");
        copied_nested.assert(predicate::path::exists());
        let content2 = fs::read_to_string(copied_nested.path()).unwrap();
        assert_eq!(content2, "bar");
    }

    #[test]
    fn test_copy_overwrites_existing_files() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("src");
        let dst = temp.child("dst");
        src.create_dir_all().unwrap();
        dst.create_dir_all().unwrap();

        // Create a file in src with "new content"
        let src_file = src.child("foo.txt");
        src_file.write_str("new content").unwrap();

        // Create a file of the same name in dst with "old content"
        let dst_file = dst.child("foo.txt");
        dst_file.write_str("old content").unwrap();

        // Perform the copy
        copy_dir_contents(src.path(), dst.path()).unwrap();

        // Assert that dst/foo.txt now contains "new content", not "old content"
        let result = std::fs::read_to_string(dst_file.path()).unwrap();
        assert_eq!(result, "new content");
    }

    #[test]
    fn errors_when_src_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("no_src");
        let dst = temp.child("dst");
        dst.create_dir_all().unwrap();

        let err = copy_dir_contents(src.path(), dst.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::PathNotFound(p)) => {
                assert_eq!(p, src.path().to_path_buf());
            }
            other => panic!("expected PathNotFound for src, got {:?}", other),
        }
    }

    #[test]
    fn errors_when_dst_missing() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("src");
        src.create_dir_all().unwrap();
        let dst = temp.child("no_dst");

        let err = copy_dir_contents(src.path(), dst.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::PathNotFound(p)) => {
                assert_eq!(p, dst.path().to_path_buf());
            }
            other => panic!("expected PathNotFound for dst, got {:?}", other),
        }
    }

    #[test]
    fn errors_when_src_is_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src_file = temp.child("file.txt");
        src_file.touch().unwrap();
        let dst = temp.child("dst");
        dst.create_dir_all().unwrap();

        let err = copy_dir_contents(src_file.path(), dst.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::NotADirectory(p)) => {
                assert_eq!(p, src_file.path().to_path_buf());
            }
            other => panic!("expected NotADirectory, for src {:?}", other),
        }
    }

    #[test]
    fn errors_when_dst_is_file() {
        let temp = assert_fs::TempDir::new().unwrap();
        let src = temp.child("src");
        src.create_dir_all().unwrap();
        let dst_file = temp.child("dst.txt");
        dst_file.touch().unwrap();

        let err = copy_dir_contents(src.path(), dst_file.path()).unwrap_err();
        match err {
            IoError::Validation(ValidationError::NotADirectory(p)) => {
                assert_eq!(p, dst_file.path().to_path_buf());
            }
            other => panic!("expected NotADirectory, for dst {:?}", other),
        }
    }
}
