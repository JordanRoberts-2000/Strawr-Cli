use std::{env, path::PathBuf};

use crate::{
    constants::{CONFIG_FOLDER_NAME, CONFIG_HOME_ENV, DEV_CONFIG_FOLDER_PATH},
    utils,
};

use super::error::StorageError;

pub fn initialize_storage_dir() -> Result<PathBuf, StorageError> {
    let base_dir = if let Ok(custom) = env::var(CONFIG_HOME_ENV) {
        log::debug!("Env variable STRAWR_HOME used: '{}'", custom);
        PathBuf::from(custom)
    } else {
        dirs::home_dir().ok_or(StorageError::HomeDirNotFound)?
    };

    let storage_dir = if cfg!(debug_assertions) && env::var(CONFIG_HOME_ENV).is_err() {
        PathBuf::from(DEV_CONFIG_FOLDER_PATH)
    } else {
        base_dir.join(CONFIG_FOLDER_NAME)
    };

    if !storage_dir.exists() {
        utils::fs::create_dir_all(&storage_dir)?;
        log::info!("Created storage directory '{:?}'", storage_dir);
    }

    Ok(storage_dir)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use tempfile::tempdir;

//     #[test]
//     fn test_get_storage_dir_uses_env_var() {
//         // Arrange
//         let temp = tempdir().expect("Failed to create temp dir");
//         let temp_path = temp.path().to_path_buf();

//         // Set the STRAWR_HOME (or whatever CONFIG_HOME_ENV is)
//         env::set_var(CONFIG_HOME_ENV, &temp_path);

//         // Act
//         let result = get_storage_dir().expect("Failed to get storage dir");

//         // Expected path: <temp_path>/<CONFIG_FOLDER_NAME>
//         let expected = temp_path.join(CONFIG_FOLDER_NAME);

//         // Assert
//         assert_eq!(result, expected);
//         assert!(expected.exists());
//     }
// }
