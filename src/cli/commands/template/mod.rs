pub mod command;
pub mod config;
pub mod error;
pub mod service;
pub mod utils;

pub use command::args::TemplateCommand;
pub use config::TemplateConfig;
pub use error::TemplateError;

pub const DEFAULT_FOLDER: &str = "default";
pub const TEMPLATES_FOLDER_NAME: &str = "templates";
