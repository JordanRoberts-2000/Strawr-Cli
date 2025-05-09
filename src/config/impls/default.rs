use validator::Validate;

use crate::{config::constants::INITIAL_CONFIG_CONTENT, error::ParseError, CliConfig};

impl Default for CliConfig {
    fn default() -> Self {
        let config: CliConfig = toml::from_str(INITIAL_CONFIG_CONTENT)
            .map_err(|e| ParseError::Toml {
                source: e,
                title: "Config file".to_string(),
            })
            .expect("Invalid INITIAL_CONFIG_CONTENT: failed to parse TOML");

        config
            .validate()
            .expect("Invalid INITIAL_CONFIG_CONTENT: validation failed");

        config
    }
}
