use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::config::constants::INITIAL_CONFIG_CONTENT;

use super::AppContext;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub grab: GrabConfig,
    pub open: OpenConfig,
}

#[derive(Debug, Deserialize)]
pub struct GrabConfig {
    pub encrypt_values_by_default: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Editor {
    VSCode,
    Nano,
    Vim,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct OpenConfig {
    pub open: Editor,
}

impl AppContext {
    pub fn parse_config(storage_dir: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = storage_dir.join("config.toml");

        if !config_path.exists() {
            fs::write(&config_path, INITIAL_CONFIG_CONTENT).map_err(|e| {
                format!("Failed to write default config to {:?}: {}", config_path, e)
            })?;
            log::debug!("Created config.toml at {:?}", config_path);
        }

        let config_contents = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file {:?}: {}", config_path, e))?;

        let config: Config = toml::from_str(&config_contents)
            .map_err(|e| format!("Failed to parse config file {:?}: {}", config_path, e))?;

        Ok(config)
    }
}
