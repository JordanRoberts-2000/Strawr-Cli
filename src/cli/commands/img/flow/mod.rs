use std::marker::PhantomData;

use image::ImageFormat;

use super::{context::ImgContext, enums::ImageInput};

use {colored::Colorize, std::path::PathBuf, walkdir::WalkDir};

use crate::{commands::img::ImgError, img::Img, validation::validate};

pub struct Start;
pub struct ImagesResolved;
pub struct ImagesTransformed;

pub struct ImgFlow<S> {
    pub _state: PhantomData<S>,
    images: Vec<Img>,
}

impl ImgFlow<Start> {
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            images: Vec::new(),
        }
    }

    pub fn resolve_images(self, input: &ImageInput) -> Result<ImgFlow<ImagesResolved>, ImgError> {
        let images = match input {
            ImageInput::Directory(path) => Self::handle_dir(path)?,
            ImageInput::File(path) => vec![Img::open(path)?],
            ImageInput::Url(url) => vec![Img::download(url)?],
        };

        Ok(ImgFlow {
            images,
            _state: PhantomData,
        })
    }

    fn handle_dir(input: &PathBuf) -> Result<Vec<Img>, ImgError> {
        let mut images: Vec<Img> = Vec::new();

        for entry in WalkDir::new(input).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if validate::existing_image_file(&path).is_ok() {
                match Img::open(&path) {
                    Ok(img) => images.push(img),
                    Err(e) => {
                        let msg = format!("failed to open img '{}', error: {e}", input.display());
                        eprintln!("{}", msg.red());
                    }
                }
            } else {
                log::debug!("Skipped: {:?}", path);
            }
        }

        if images.is_empty() {
            return Err(ImgError::NoImagesFilesFound(input.clone()));
        }

        Ok(images)
    }
}

impl ImgFlow<ImagesResolved> {
    pub fn apply_transformations(
        mut self,
        ctx: &ImgContext,
    ) -> Result<ImgFlow<ImagesTransformed>, ImgError> {
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

impl ImgFlow<ImagesTransformed> {
    pub fn save(self, output: &Option<Option<PathBuf>>) -> Result<(), ImgError> {
        for img in self.images.iter() {
            img.save_to("./playground/refactor")?;
        }

        Ok(())
    }
}
