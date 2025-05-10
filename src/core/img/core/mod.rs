use std::path::PathBuf;

pub mod conversions;
pub mod extractors;
pub mod initialization;
pub mod modifiers;
pub mod save;
pub mod transformations;

pub struct Img {
    img: image::DynamicImage,
    src: ImgSrc,
    pub target_path: PathBuf,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: image::ImageFormat,
    pub size_bytes: usize,
}

pub enum ImgSrc {
    Local { path: PathBuf },
    Remote { url: url::Url },
    Bytes { id: String },
}
