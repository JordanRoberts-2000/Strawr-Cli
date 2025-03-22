pub mod conversions;
pub mod creation;
pub mod enums;
pub mod error;
pub mod save;
pub mod transformations;

pub use enums::compression_type::CompressionType;

pub struct Img {
    img: image::DynamicImage,
    path: std::path::PathBuf,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: image::ImageFormat,
    pub size_bytes: usize,
}
