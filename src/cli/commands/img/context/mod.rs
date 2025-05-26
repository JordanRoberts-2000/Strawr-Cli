use std::path::{Path, PathBuf};

use image::ImageFormat;

use crate::{img::CompressionType, CliConfig};

use super::{enums::ValidImageFormat, ImgCommand};

pub struct ImgContext {
    pub blur_intensity: Option<u8>,
    pub max_size: Option<u32>,
    target_format: ValidImageFormat,
    quality: ImgQuality,
    file_name: ImgFileName,
    pub webp_lossy: bool,
}

pub struct ImgFileName {
    rename_to: Option<String>,
    suffix: Option<String>,
    prefix: Option<String>,
}

struct ImgQuality {
    arg: Option<u8>,
    default: u8,
    default_webp: u8,
    default_jpeg: u8,
}
const DEFAULT_QUALITY: u8 = 100;

impl ImgContext {
    pub fn new(args: &ImgCommand, cfg: &CliConfig) -> Self {
        let format = args.format.as_ref().unwrap_or(&cfg.img.default_format);

        Self {
            blur_intensity: Self::handle_blur(&args, &cfg),
            max_size: Self::handle_max_size(&args, &cfg),
            target_format: format.clone(),
            quality: ImgQuality {
                arg: args.quality.clone(),
                default: DEFAULT_QUALITY,
                default_webp: cfg.img.default_webp_quality.clone(),
                default_jpeg: cfg.img.default_jpg_quality.clone(),
            },
            file_name: ImgFileName {
                rename_to: args.rename.clone(),
                prefix: args.prefix.clone(),
                suffix: args.suffix.clone(),
            },
            webp_lossy: matches!(cfg.img.default_webp_compression, CompressionType::Lossy),
        }
    }

    pub fn get_format(&self, format: &ImageFormat) -> ImageFormat {
        self.target_format.clone().try_into().unwrap_or(*format)
    }

    pub fn get_quality(&self, format: &ImageFormat) -> u8 {
        self.quality.arg.unwrap_or_else(|| match format {
            ImageFormat::Jpeg => self.quality.default_jpeg,
            ImageFormat::WebP => self.quality.default_webp,
            _ => self.quality.default,
        })
    }

    pub fn handle_renaming(&self, current_stem: &str) -> String {
        let base = self
            .file_name
            .rename_to
            .as_ref()
            .cloned()
            .unwrap_or_else(|| current_stem.to_string());

        // 2) apply prefix
        let with_prefix = self
            .file_name
            .prefix
            .as_ref()
            .map(|p| format!("{}{}", p, base))
            .unwrap_or(base.clone());

        // 3) apply suffix
        let with_suffix = self
            .file_name
            .suffix
            .as_ref()
            .map(|s| format!("{}{}", with_prefix, s))
            .unwrap_or(with_prefix);

        with_suffix
    }

    fn handle_blur(args: &ImgCommand, cfg: &CliConfig) -> Option<u8> {
        if let Some(blur) = args.blur {
            Some(blur.unwrap_or(cfg.img.placeholder_blur_intensity))
        } else if args.placeholder.unwrap_or(false) {
            Some(cfg.img.placeholder_blur_intensity)
        } else {
            None
        }
    }

    fn handle_max_size(args: &ImgCommand, cfg: &CliConfig) -> Option<u32> {
        match &args.size {
            Some(size) => Some(size.clone().to_pixels()),
            None if args.placeholder.unwrap_or(false) => Some(cfg.img.placeholder_size),
            None if args.thumbnail.unwrap_or(false) => Some(cfg.img.thumbnail_size),
            None => cfg.img.max_size,
        }
    }
}
