use std::path::PathBuf;

use image::ImageFormat;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::{CompressionType, Img},
    state::AppContext,
};

impl ImgCommand {
    pub fn process_image(&self, img: &mut Img, ctx: &AppContext) -> Result<PathBuf, ImgError> {
        // handle blur
        if let Some(blur) = &self.blur {
            let blur_intensity = match blur {
                None => ctx.config.img.blur_intensity,
                Some(val) => *val,
            };
            img.blur(blur_intensity);
        }

        // handle quality + format
        let format = match &self.format {
            Some(format) => format.to_image_format().unwrap_or(img.format),
            None => ctx
                .config
                .img
                .default_format
                .to_image_format()
                .unwrap_or(img.format),
        };

        const DEFAULT_QUALITY: u8 = 100;
        let quality = self.quality.unwrap_or_else(|| match format {
            ImageFormat::Jpeg => ctx.config.img.default_jpg_quality,
            ImageFormat::WebP => ctx.config.img.default_webp_quality,
            _ => DEFAULT_QUALITY,
        });

        if format != img.format || (quality != 100 && format != ImageFormat::Png) {
            match format {
                ImageFormat::Jpeg => {
                    log::trace!("Jpeg conversion with quality '{quality}' successful");
                    img.jpeg(quality)?;
                }
                ImageFormat::Png => {
                    log::trace!("Png conversion successful");
                    img.png()?;
                }
                ImageFormat::WebP => {
                    let is_lossy = matches!(
                        ctx.config.img.default_webp_compression,
                        CompressionType::Lossy
                    );
                    if is_lossy || quality != 100 {
                        log::trace!("Lossy Webp conversion with quality '{quality}' successful");
                        img.webp_lossy(quality)?;
                    } else {
                        log::trace!("Lossless Webp conversion successful");
                        img.webp()?;
                    }
                }
                _ => {
                    log::warn!("Unsupported format for re-encoding: {:?}", format);
                }
            }
        }

        Ok(PathBuf::new())
    }
}
