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
    Remote { url: url::Url, target: PathBuf },
}

impl Img {
    pub fn file_name(&mut self, new_name: &str) -> &mut Self {
        match &mut self.src {
            ImgSrc::Local { target, .. } | ImgSrc::Remote { target, .. } => {
                *target = target.with_file_name(new_name);
            }
        }
        self
    }

    pub fn update_extension(&mut self, new_ext: &str) {
        match &mut self.src {
            ImgSrc::Local { target, .. } | ImgSrc::Remote { target, .. } => {
                *target = target.with_extension(new_ext);
            }
        }
    }
}
