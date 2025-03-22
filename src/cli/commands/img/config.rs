use validator::Validate;

use crate::services::img::CompressionType;

#[derive(Debug, serde::Deserialize, Validate)]
pub struct ImgConfig {
    // pub default_format: ValidImageFormat,
    pub default_webp_compression: CompressionType,
    #[validate(range(min = 1, max = 100, message = "Must be between 1 and 100"))]
    pub default_webp_quality: u8,
    #[validate(range(min = 1, max = 100, message = "Must be between 1 and 100"))]
    pub default_jpg_quality: u8,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub thumbnail_size: u32,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub blur_size: u32,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub blur_intensity: u32,
    pub deleted_images_to_recycling_bin: bool,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub max_size: Option<u32>,
}
