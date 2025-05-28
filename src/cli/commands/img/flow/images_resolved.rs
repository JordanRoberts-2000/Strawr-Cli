use std::marker::PhantomData;

use image::ImageFormat;

use crate::commands::img::{context::ImgContext, ImgCmdError};

use super::{
    core::{ImagesResolved, ImagesTransformed},
    ImgFlow,
};

impl ImgFlow<ImagesResolved> {
    pub fn apply_transformations(
        mut self,
        ctx: &ImgContext,
    ) -> Result<ImgFlow<ImagesTransformed>, ImgCmdError> {
        for img in self.images.iter_mut() {
            let format = ctx.get_format(&img.format);
            let quality = ctx.get_quality(&img.format);

            if let Some(intensity) = ctx.blur_intensity {
                img.blur(intensity);
                log::info!("Blur applied to image with intensity: {}", intensity);
            }

            if let Some(max_size) = &ctx.max_size {
                if img.width > *max_size || img.height > *max_size {
                    img.max_size(*max_size);
                    log::info!("Image resized with max size {max_size}");
                } else {
                    log::debug!(
                        "Skipping resize: image already within max size ({}x{}) ≤ {}",
                        img.width,
                        img.height,
                        max_size
                    );
                }
            }

            if format != img.format || (quality != 100 && format != ImageFormat::Png) {
                match format {
                    ImageFormat::Jpeg => {
                        img.jpeg(quality)?;
                        log::info!("Jpeg conversion applied with quality '{quality}'");
                    }
                    ImageFormat::Png => {
                        img.png()?;
                        log::info!("Png conversion applied");
                    }
                    ImageFormat::WebP => {
                        if ctx.webp_lossy || quality != 100 {
                            img.webp_lossy(quality)?;
                            log::info!("Lossy Webp conversion applied with quality '{quality}'");
                        } else {
                            img.webp()?;
                            log::info!("Lossless Webp conversion applied");
                        }
                    }
                    _ => {
                        log::warn!("Unsupported format for re-encoding: {:?}", format);
                    }
                }
            }

            let original_file_name = img.file_stem()?;
            let new_file_name = ctx.handle_renaming(&original_file_name);

            if original_file_name != new_file_name {
                img.with_file_name(&new_file_name)?;
                log::info!("New filename '{new_file_name}' applied to image");
            }
        }

        Ok(ImgFlow {
            images: self.images,
            _state: PhantomData,
        })
    }
}
