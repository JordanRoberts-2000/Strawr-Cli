use validator::Validate;

use super::utils::valid_image_format::ValidImageFormat;

#[derive(Debug, serde::Deserialize, Validate)]
pub struct ImgConfig {
    pub default_format: ValidImageFormat,
    pub max_size: Option<u32>,
}
