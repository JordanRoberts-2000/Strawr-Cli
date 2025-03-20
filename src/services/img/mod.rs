use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

pub mod conversions;
pub mod enums;
pub mod transformations;

pub use enums::compression_type::CompressionType;

pub struct Img {
    img: DynamicImage,
    path: PathBuf,
    height: u32,
    width: u32,
}

impl Img {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        Ok(Self {
            img,
            path: path.clone(),
            height,
            width,
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
