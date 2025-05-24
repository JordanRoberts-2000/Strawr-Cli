use crate::{
    config::{constants::INITIAL_CONFIG_CONTENT, CliConfig, ConfigError},
    error::{IoError, ParseError},
};
use {
    std::{fs, path::PathBuf},
    validator::Validate,
};

impl CliConfig {
    pub fn parse(storage_dir: &PathBuf) -> Result<(Self, PathBuf), ConfigError> {
        let config_path = storage_dir.join("config.toml");
        log::trace!("Config path: {:?}", config_path);

        if !config_path.exists() {
            fs::write(&config_path, INITIAL_CONFIG_CONTENT)
                .map_err(|e| IoError::WriteFile(e, config_path.clone()))?;
            log::debug!("Created config.toml at {:?}", config_path);
        }

        let config_contents = fs::read_to_string(&config_path)
            .map_err(|e| IoError::ReadFile(e, config_path.clone()))?;

        let config: CliConfig = toml::from_str(&config_contents).map_err(|e| ParseError::Toml {
            source: e,
            title: "Config file".to_string(),
        })?;

        config.validate().map_err(ConfigError::Validation)?;

        Ok((config, config_path))
    }
}
