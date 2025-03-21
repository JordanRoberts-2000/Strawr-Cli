use crate::services::img::Img;
use image::{codecs::jpeg::JpegEncoder, load_from_memory, ExtendedColorType};

impl Img {
    pub fn jpeg_convert(&mut self, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

        encoder.encode(
            self.img.to_rgb8().as_raw(),
            self.width,
            self.height,
            ExtendedColorType::Rgb8,
        )?;
        // Note: This decoding will re-read the JPEG data, so any JPEG-specific encoding
        // (like progressive settings) will be lost.
        self.img = load_from_memory(&buffer)?;
        Ok(())
    }
}
