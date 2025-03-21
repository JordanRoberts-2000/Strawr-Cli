use image::{load_from_memory, ImageFormat};
use std::io::Cursor;

use crate::services::img::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn png(&mut self) -> Result<&mut Self> {
        let mut buffer = Vec::new();
        self.img
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .map_err(|e| ImgError::Conversion {
                err_string: format!("{:?}", e),
                path: self.path.clone(),
                format: ImageFormat::Png,
            })?;

        self.img = load_from_memory(&buffer).map_err(|e| ImgError::ImageDecoding {
            path: self.path.clone(),
            source: e,
            format: ImageFormat::Png,
        })?;

        Ok(self)
    }
}
