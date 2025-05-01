use validator::Validate;

use crate::error::ParseError;

use super::{Config, INITIAL_CONFIG_CONTENT};

impl Default for Config {
    fn default() -> Self {
        let config: Config = toml::from_str(INITIAL_CONFIG_CONTENT)
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
