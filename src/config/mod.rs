use serde::Deserialize;
use validator::Validate;

use crate::{
    cli::commands::{grab::GrabConfig, img::ImgConfig, template::TemplateConfig},
    utils::Editor,
};

pub mod default;
pub mod error;
pub mod parse;

pub const INITIAL_CONFIG_CONTENT: &str = include_str!("initial_config.toml");

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub default_editor: Editor,
    pub grab: GrabConfig,
    #[validate(nested)]
    pub img: ImgConfig,
    pub template: TemplateConfig,
}
