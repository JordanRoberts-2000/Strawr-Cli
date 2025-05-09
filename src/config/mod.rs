mod config;
mod error;
mod impls {
    mod default;
    mod parse;
}

pub(crate) mod constants {
    pub const INITIAL_CONFIG_CONTENT: &str = include_str!("../assets/initial_config.toml");
}

pub use config::CliConfig;
pub(crate) use error::ConfigError;
