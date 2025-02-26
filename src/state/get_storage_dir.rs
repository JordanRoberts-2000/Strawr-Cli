use std::{env, fs, path::PathBuf};

use crate::config::constants::{CONFIG_FOLDER_NAME, CONFIG_HOME_ENV, DEV_CONFIG_FOLDER_NAME};

use super::AppContext;

impl AppContext {
    pub fn get_storage_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let base_dir = if let Ok(custom) = env::var("STRAWR_HOME") {
            log::debug!("Env variable STRAWR_HOME used: '{}'", custom);

            let custom_dir = PathBuf::from(custom);
            if !custom_dir.exists() {
                fs::create_dir_all(&custom_dir).map_err(|e| {
                    format!(
                        "Failed to create STRAWR_HOME directory {:?}: {}",
                        custom_dir, e
                    )
                })?;
                log::debug!("Created dirs for STRAWR_HOME path successfully");
            }
            custom_dir
        } else if let Some(home) = dirs::home_dir() {
            home
        } else {
            return Err(format!(
                "Could not determine home directory. Please set the {} environment variable.",
                CONFIG_HOME_ENV
            )
            .into());
        };

        let folder_name = if cfg!(debug_assertions) {
            DEV_CONFIG_FOLDER_NAME
        } else {
            CONFIG_FOLDER_NAME
        };

        let storage_dir = base_dir.join(folder_name);

        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir).map_err(|e| {
                format!(
                    "Failed to create storage directory {:?}: {}",
                    storage_dir, e
                )
            })?;
            log::debug!("Created storage directory '{:?}' successfully", storage_dir);
        }

        Ok(storage_dir)
    }
}
