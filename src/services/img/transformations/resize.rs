use crate::services::img::Img;
use image::imageops::FilterType;

impl Img {
    pub fn max_size(&mut self, max_size: u32) {
        if self.width <= max_size && self.height <= max_size {
            return;
        }

        let scale = if self.width > self.height {
            max_size as f32 / self.width as f32
        } else {
            max_size as f32 / self.height as f32
        };

        let new_width = (self.width as f32 * scale).round() as u32;
        let new_height = (self.height as f32 * scale).round() as u32;

        self.img = self.img.resize(new_width, new_height, FilterType::Lanczos3);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.img = self.img.resize(width, height, FilterType::Lanczos3);
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) {
        self.img = self.img.resize_exact(width, height, FilterType::Lanczos3);
    }
}
