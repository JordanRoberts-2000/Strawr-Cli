use crate::services::img::{
    error::{ImgError, Result},
    Img,
};
use image::{codecs::jpeg::JpegEncoder, load_from_memory, ExtendedColorType, ImageFormat};

impl Img {
    pub fn jpeg(&mut self, quality: u8) -> Result<&mut Self> {
        let mut buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

        encoder
            .encode(
                self.img.to_rgb8().as_raw(),
                self.width,
                self.height,
                ExtendedColorType::Rgb8,
            )
            .map_err(|e| ImgError::Conversion {
                err_string: format!("{:?}", e),
                id: self.id(),
                format: ImageFormat::Jpeg,
            })?;

        // Note: This decoding will re-read the JPEG data, so any JPEG-specific encoding
        // (like progressive settings) will be lost.
        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.id(),
            source: e,
            format: ImageFormat::Jpeg,
        })?;

        self.format = ImageFormat::Jpeg;
        Ok(self)
    }
}
