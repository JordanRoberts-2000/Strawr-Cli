use std::{fs, path::PathBuf};

use image::{guess_format, GenericImageView};

use super::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let img = image::open(path).map_err(|e| ImgError::Open {
            source: e,
            path: path.clone(),
        })?;

        let (width, height) = img.dimensions();

        let bytes = fs::read(path).map_err(|e| ImgError::Io {
            context: format!("failed to read '{:?}'", path),
            source: e,
        })?;

        let size_bytes = bytes.len();
        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;

        Ok(Self {
            img,
            path: path.clone(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
            size_bytes,
        })
    }
}
