use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use validator::Validate;

use crate::commands::grab::config::GrabConfig;
use crate::commands::img::config::ImgConfig;
use crate::config::constants::INITIAL_CONFIG_CONTENT;
use crate::error::{Error, ParseError, Result};

use super::AppContext;

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub grab: GrabConfig,
    #[validate(nested)]
    pub img: ImgConfig,
    #[validate(nested)]
    pub open: OpenConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Editor {
    VSCode,
    Nano,
    Vim,
    Path,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OpenConfig {
    pub editor: Editor,
}

impl AppContext {
    pub fn parse_config(storage_dir: &PathBuf) -> Result<Config> {
        let config_path = storage_dir.join("config.toml");

        if !config_path.exists() {
            fs::write(&config_path, INITIAL_CONFIG_CONTENT)
                .map_err(|e| Error::Io(e, "config.toml could not be initialized".to_string()))?;
            log::debug!("Created config.toml at {:?}", config_path);
        }

        let config_contents = fs::read_to_string(&config_path)
            .map_err(|e| Error::Io(e, format!("Failed to read config file '{:?}'", config_path)))?;

        let config: Config = toml::from_str(&config_contents).map_err(|e| {
            Error::Parse(
                ParseError::Toml(e),
                "Failed to parse config.toml".to_string(),
            )
        })?;

        config
            .validate()
            .map_err(|e| Error::Validation(e, "config.toml".to_string()))?;

        Ok(config)
    }
}
