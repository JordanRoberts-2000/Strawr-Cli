use crate::error::{Error, Result};
use image::{ImageFormat, ImageReader};
use std::path::PathBuf;

pub struct ImageService {
    format: ImageFormat,
}

impl ImageService {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let img_reader = ImageReader::open(path)
            .map_err(|e| Error::Custom(format!("Failed to open image, {}", e)))?
            .with_guessed_format()
            .map_err(|e| Error::Custom(format!("Failed to guess image format: {}", e)))?;

        if let Some(fmt) = img_reader.format() {
            if matches!(
                fmt,
                ImageFormat::Avif | ImageFormat::WebP | ImageFormat::Jpeg | ImageFormat::Png
            ) {
                Ok(Self { format: fmt })
            } else {
                Err(Error::Custom(format!("Unsupported format: {:?}", fmt)))
            }
        } else {
            Err(Error::Custom("Could not detect format".to_string()))
        }
    }
}
