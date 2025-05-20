use std::{fs, path::Path};

use crate::error::IoError;

use super::ensure_dir;

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

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use super::*;
//     use tempfile::tempdir;

//     #[test]
//     fn returns_only_subfolders() -> Result<(), Box<dyn std::error::Error>> {
//         let temp_dir = tempdir()?;

//         // Create subfolders
//         let subfolder1 = temp_dir.path().join("folder1");
//         let subfolder2 = temp_dir.path().join("folder2");
//         fs::create_dir(&subfolder1)?;
//         fs::create_dir(&subfolder2)?;

//         // Create files
//         let file1 = temp_dir.path().join("file1.txt");
//         fs::write(&file1, "content")?;

//         let result = subfolders(temp_dir.path())?;

//         // Should only return the two folders
//         assert_eq!(result.len(), 2);
//         assert!(result.contains(&"folder1".to_string()));
//         assert!(result.contains(&"folder2".to_string()));
//         assert!(!result.contains(&"file1.txt".to_string()));

//         Ok(())
//     }

//     #[test]
//     fn returns_empty_for_empty_dir() -> Result<(), Box<dyn std::error::Error>> {
//         let temp_dir = tempdir()?;
//         let result = subfolders(temp_dir.path())?;
//         assert!(result.is_empty());
//         Ok(())
//     }

//     #[test]
//     fn returns_error_for_nonexistent_path() {
//         let non_existent_path = PathBuf::from("/unlikely_to_exist_test_path_xyz123");
//         let result = subfolders(non_existent_path);
//         assert!(result.is_err());
//     }
// }
