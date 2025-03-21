use image::{load_from_memory, ImageFormat};
use std::io::Cursor;

use crate::services::img::Img;

impl Img {
    pub fn png_convert(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        self.img
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        self.img = load_from_memory(&buffer)?;
        Ok(())
    }
}
