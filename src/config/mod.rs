use serde::Deserialize;
use validator::Validate;

use crate::{
    cli::commands::{grab::GrabConfig, img::ImgConfig},
    utils::Editor,
};

pub mod error;
pub mod parse;

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub default_editor: Editor,
    pub grab: GrabConfig,
    #[validate(nested)]
    pub img: ImgConfig,
    // #[validate(nested)]
    // pub open: OpenConfig,
}
