pub(crate) mod constants {
    pub const DEFAULT_FOLDER: &str = "default";
    pub const TEMPLATES_FOLDER_NAME: &str = "templates";
}

pub mod command;
mod config;
pub(crate) mod controller;
mod error;
pub(crate) mod models;
pub(crate) mod service;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod view;

pub use self::{command::sub_commands, command::TemplateCommand, error::TemplateError};
pub(crate) use self::{
    command::TemplateSubcommand, config::TemplateConfig, controller::TemplateController,
    service::TemplateService, view::TemplateView,
};
