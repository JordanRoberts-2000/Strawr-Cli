use crate::services::img::Img;
use image::ImageFormat;

impl Img {
    pub fn get_extention(&self) -> Option<&'static str> {
        match self.format {
            ImageFormat::Jpeg => Some("jpg"),
            ImageFormat::WebP => Some("webp"),
            ImageFormat::Png => Some("png"),
            _ => None,
        }
    }
}
