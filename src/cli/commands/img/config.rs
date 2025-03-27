use crate::services::img::CompressionType;

use super::utils::enums::{ImageSize, ValidImageFormat};

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorOutput {
    Rgb,
    Hex,
}

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgConfig {
    pub default_format: ValidImageFormat,
    pub default_webp_compression: CompressionType,
    #[validate(range(min = 1, max = 100, message = "Must be between 1 and 100"))]
    pub default_webp_quality: u8,
    #[validate(range(min = 1, max = 100, message = "Must be between 1 and 100"))]
    pub default_jpg_quality: u8,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub thumbnail_size: u32,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub blur_intensity: u8,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub placeholder_size: u32,
    pub placeholder_blur_intensity: u8,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub max_size: Option<u32>,
    pub default_get_color_output: ColorOutput,
    #[validate(range(min = 2, max = 3, message = "Must be either 2 or 3"))]
    pub default_dalle_version: u8,
    pub default_dalle_size: ImageSize,
}
