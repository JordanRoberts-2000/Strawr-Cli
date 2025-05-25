use crate::ai::{AiImageModel, ImageSize};

use super::{enums::ColorOutput, utils::dalle_version_deserialize};

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgConfig {
    // pub default_format: ValidImageFormat,
    // pub default_webp_compression: CompressionType,
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
    pub get: ImgGetConfig,
    pub gen: ImgGenConfig,
}

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgGetConfig {
    pub default_color_output: ColorOutput,
}

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgGenConfig {
    #[serde(deserialize_with = "dalle_version_deserialize")]
    pub prefered_dalle_version: AiImageModel,
    pub default_img_size: ImageSize,
}
