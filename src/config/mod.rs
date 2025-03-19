use serde::Deserialize;
use validator::Validate;

use crate::cli::commands::{grab::GrabConfig, img::ImgConfig};

pub mod parse;

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub grab: GrabConfig,
    #[validate(nested)]
    pub img: ImgConfig,
    // #[validate(nested)]
    // pub open: OpenConfig,
}
