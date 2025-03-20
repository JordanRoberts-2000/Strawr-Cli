use validator::Validate;

use crate::services::img::CompressionType;

#[derive(Debug, serde::Deserialize, Validate)]
pub struct ImgConfig {
    // pub default_format: ValidImageFormat,
    #[validate(range(min = 1, max = 10, message = "Must be between 1 and 10"))]
    pub default_avif_speed: u8,
    pub default_avif_compression: CompressionType,
    pub default_webp_compression: CompressionType,
    #[validate(range(min = 1, max = 100, message = "Must be between 1 and 100"))]
    pub default_quality: u8,
    #[validate(range(min = 1, message = "Must be greater than 0"))]
    pub max_size: Option<u32>,
}
