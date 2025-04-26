pub mod command;
pub mod config;
pub mod error;

pub use command::args::TemplateCommand;
pub use config::TemplateConfig;
pub use error::TemplateError;

pub const DEFAULT_FOLDER: &str = "default";
