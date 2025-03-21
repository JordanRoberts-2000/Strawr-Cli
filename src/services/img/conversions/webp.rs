use image::ImageFormat;
use webp::{Encoder, PixelLayout};

use crate::services::img::Img;

impl Img {
    pub fn webp_convert(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Convert the image to RGBA8, which is what the webp encoder expects.
        let rgba_image = self.img.to_rgba8();

        // Create a WebP encoder.
        let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

        let webp_data = encoder.encode_simple(false, 100.0).expect("handle error");

        self.img = image::load_from_memory_with_format(&webp_data, ImageFormat::WebP)?;
        Ok(())
    }
}
