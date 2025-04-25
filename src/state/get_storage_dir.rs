use std::{env, fs, path::PathBuf};

use crate::{
    constants::{CONFIG_FOLDER_NAME, CONFIG_HOME_ENV, DEV_CONFIG_FOLDER_PATH},
    error::IoError,
};

use super::{error::StateError, AppContext};

impl AppContext {
    pub fn get_storage_dir() -> Result<PathBuf, StateError> {
        let base_dir = if let Ok(custom) = env::var("STRAWR_HOME") {
            log::debug!("Env variable STRAWR_HOME used: '{}'", custom);

            let custom_dir = PathBuf::from(custom);
            if !custom_dir.exists() {
                fs::create_dir_all(&custom_dir)
                    .map_err(|e| IoError::CreateDir(e, custom_dir.clone()))?;
                log::debug!("Created dirs for STRAWR_HOME path successfully");
            }
            custom_dir
        } else if let Some(home) = dirs::home_dir() {
            home
        } else {
            return Err(StateError::HomeDirNotFound {
                env_var: CONFIG_HOME_ENV,
            });
        };

        let storage_dir = if cfg!(debug_assertions) {
            PathBuf::from(DEV_CONFIG_FOLDER_PATH)
        } else {
            base_dir.join(CONFIG_FOLDER_NAME)
        };

        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)
                .map_err(|e| IoError::CreateDir(e, storage_dir.clone()))?;
            log::info!("Created storage directory '{:?}' successfully", storage_dir);
        }

        Ok(storage_dir)
    }
}
