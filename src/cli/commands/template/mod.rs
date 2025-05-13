pub(crate) mod constants {
    pub const DEFAULT_FOLDER: &str = "default";
    pub const TEMPLATES_FOLDER_NAME: &str = "templates";
}

pub(crate) mod types {
    pub type TemplateInput = (String, Option<String>);
    pub type VariantInput = Option<Option<String>>;
}

pub mod command;
mod config;
mod context;
pub(crate) mod controller;
mod error;
pub(crate) mod models;
pub(crate) mod service;
pub(crate) mod utils;
pub(crate) mod view;

pub use self::{command::sub_commands, command::TemplateCommand, error::TemplateError};
pub(crate) use self::{
    command::TemplateSubcommand, config::TemplateConfig, context::TemplateContext,
    controller::TemplateController, service::TemplateService, view::TemplateView,
};
