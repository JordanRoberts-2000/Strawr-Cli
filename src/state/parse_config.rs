use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use validator::{Validate, ValidationError};

use crate::config::constants::INITIAL_CONFIG_CONTENT;
use crate::error::{Error, ParseError, Result};

use super::AppContext;

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub grab: GrabConfig,
    #[validate(nested)]
    pub open: OpenConfig,
}

#[derive(Debug, Deserialize)]
pub struct GrabConfig {
    pub encrypt_values_by_default: bool,
}

fn validate_editor(editor: &Editor) -> std::result::Result<(), ValidationError> {
    if let Editor::Unknown = editor {
        let mut err = ValidationError::new("invalid_editor");
        err.message = Some("Editor must be one of 'vscode', 'nano', 'path' or 'vim'".into());
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Editor {
    VSCode,
    Nano,
    Vim,
    Path,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OpenConfig {
    #[validate(custom(function = "validate_editor"))]
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
