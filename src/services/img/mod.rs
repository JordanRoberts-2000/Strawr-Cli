use image::{guess_format, DynamicImage, GenericImageView, ImageFormat};
use std::{fs, path::PathBuf};

pub mod conversions;
pub mod enums;
pub mod error;
pub mod transformations;

pub use enums::compression_type::CompressionType;

pub struct Img {
    img: DynamicImage,
    path: PathBuf,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
}

impl Img {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();

        let bytes = fs::read(path)?;
        let format = guess_format(&bytes)?;

        Ok(Self {
            img,
            path: path.clone(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.img.save(&self.path)?;
        Ok(())
    }

    pub fn save_to(&self, output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.img.save(output_path)?;
        Ok(())
    }
}
