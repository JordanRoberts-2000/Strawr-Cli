use std::path::PathBuf;

pub mod conversions;
pub mod extractors;
pub mod initialization;
pub mod save;
pub mod transformations;

pub struct Img {
    img: image::DynamicImage,
    src: ImgSrc,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: image::ImageFormat,
    pub size_bytes: usize,
}

pub enum ImgSrc {
    Local { path: PathBuf, target: PathBuf },
    Remote { url: String, target: PathBuf },
}

impl Img {
    pub fn id(&self) -> String {
        match &self.src {
            ImgSrc::Local { path, .. } => path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string_lossy().into_owned()),
            ImgSrc::Remote { url, .. } => url.clone(),
        }
    }
}
