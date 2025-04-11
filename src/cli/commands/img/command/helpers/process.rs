use std::path::PathBuf;

use image::ImageFormat;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::{CompressionType, Img},
    state::AppContext,
};

impl ImgCommand {
    pub fn process_image(&self, img: &mut Img, ctx: &AppContext) -> Result<PathBuf, ImgError> {
        self.apply_blur(img, ctx);

        self.apply_resize(img, ctx);

        self.apply_conversion(img, ctx)?;

        Ok(PathBuf::new())
    }

    fn apply_blur(&self, img: &mut Img, ctx: &AppContext) {
        if let Some(blur) = &self.blur {
            img.blur(blur.unwrap_or(ctx.config.img.blur_intensity));
        } else if self.placeholder.unwrap_or(false) {
            img.blur(ctx.config.img.placeholder_blur_intensity);
        }
    }

    fn apply_resize(&self, img: &mut Img, ctx: &AppContext) {
        if let Some(size) = &self.size {
            let resize_to: u32 = size.into();
            img.resize(resize_to, resize_to);
        } else if let Some(size) = self.max.or(ctx.config.img.max_size) {
            img.resize(size, size);
        } else if self.placeholder.unwrap_or(false) {
            img.resize(
                ctx.config.img.placeholder_size,
                ctx.config.img.placeholder_size,
            );
        } else if self.thumbnail.unwrap_or(false) {
            img.resize(ctx.config.img.thumbnail_size, ctx.config.img.thumbnail_size);
        }
    }

    fn apply_conversion(&self, img: &mut Img, ctx: &AppContext) -> Result<(), ImgError> {
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
