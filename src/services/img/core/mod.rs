pub mod conversions;
pub mod creation;
pub mod extractors;
pub mod save;
pub mod transformations;

pub struct Img {
    img: image::DynamicImage,
    path: std::path::PathBuf,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: image::ImageFormat,
    pub size_bytes: usize,
}
