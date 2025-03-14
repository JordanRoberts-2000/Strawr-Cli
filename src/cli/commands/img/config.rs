use validator::Validate;

use super::utils::valid_image_format::ValidImageFormat;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum CompressionType {
    Lossy,
    Lossless,
}

#[derive(Debug, serde::Deserialize, Validate)]
pub struct ImgConfig {
    pub default_format: ValidImageFormat,
    pub default_avif_speed: u8,
    pub default_avif_compression: CompressionType,
    pub default_webp_compression: CompressionType,
    pub default_quality: u8,
    pub max_size: Option<u32>,
}
