use image::{load_from_memory, ImageFormat};
use std::io::Cursor;

use crate::services::img::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn png(&mut self) -> Result<&mut Self> {
        if self.format == ImageFormat::Png {
            return Ok(self);
        }

        let mut buffer = Vec::new();
        self.img
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.id(),
                format: ImageFormat::Png,
            })?;

        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.id(),
            source: e,
            format: ImageFormat::Png,
        })?;

        self.format = ImageFormat::Png;
        self.update_extension("png");

        Ok(self)
    }
}
