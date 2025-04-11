use image::ImageFormat;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::{CompressionType, Img},
    state::AppContext,
};

impl ImgCommand {
    pub fn apply_conversion(&self, img: &mut Img, ctx: &AppContext) -> Result<(), ImgError> {
        let format = match &self.format {
            Some(format) => format.try_into(),
            None => (&ctx.config.img.default_format).try_into(),
        }
        .unwrap_or(img.format);

        const DEFAULT_QUALITY: u8 = 100;
        let quality = self.quality.unwrap_or_else(|| match format {
            ImageFormat::Jpeg => ctx.config.img.default_jpg_quality,
            ImageFormat::WebP => ctx.config.img.default_webp_quality,
            _ => DEFAULT_QUALITY,
        });

        if format != img.format || (quality != 100 && format != ImageFormat::Png) {
            match format {
                ImageFormat::Jpeg => {
                    img.jpeg(quality)?;
                    log::trace!("Jpeg conversion with quality '{quality}' successful");
                }
                ImageFormat::Png => {
                    img.png()?;
                    log::trace!("Png conversion successful");
                }
                ImageFormat::WebP => {
                    let is_lossy = matches!(
                        ctx.config.img.default_webp_compression,
                        CompressionType::Lossy
                    );
                    if is_lossy || quality != 100 {
                        img.webp_lossy(quality)?;
                        log::trace!("Lossy Webp conversion with quality '{quality}' successful");
                    } else {
                        img.webp()?;
                        log::trace!("Lossless Webp conversion successful");
                    }
                }
                _ => {
                    log::warn!("Unsupported format for re-encoding: {:?}", format);
                }
            }
        }

        Ok(())
    }
}
