use std::str::FromStr;

use image::ImageFormat;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    #[serde(alias = "jpg")]
    Jpeg,
    Webp,
    Png,
    Original,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ImageFormatConversionError {
    #[error("ValidImageFormat::Original does not map to a concrete ImageFormat")]
    NoConcreteFormat,
}

impl TryFrom<ValidImageFormat> for ImageFormat {
    type Error = ImageFormatConversionError;

    fn try_from(value: ValidImageFormat) -> Result<Self, Self::Error> {
        match value {
            ValidImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ValidImageFormat::Png => Ok(ImageFormat::Png),
            ValidImageFormat::Webp => Ok(ImageFormat::WebP),
            ValidImageFormat::Original => Err(ImageFormatConversionError::NoConcreteFormat),
        }
    }
}

impl TryFrom<&ValidImageFormat> for ImageFormat {
    type Error = ImageFormatConversionError;

    fn try_from(value: &ValidImageFormat) -> Result<Self, Self::Error> {
        match value {
            ValidImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ValidImageFormat::Png => Ok(ImageFormat::Png),
            ValidImageFormat::Webp => Ok(ImageFormat::WebP),
            ValidImageFormat::Original => Err(ImageFormatConversionError::NoConcreteFormat),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Size {
    Sm,
    Md,
    Lg,
    Pixels(u32),
}

impl FromStr for Size {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "sm" => Ok(Size::Sm),
            "md" => Ok(Size::Md),
            "lg" => Ok(Size::Lg),
            _ => input
                .parse::<u32>()
                .map(Size::Pixels)
                .map_err(|_| format!("invalid size: {input}")),
        }
    }
}

impl Into<u32> for Size {
    fn into(self) -> u32 {
        match self {
            Size::Sm => 640,
            Size::Md => 768,
            Size::Lg => 1024,
            Size::Pixels(px) => px,
        }
    }
}

impl Into<u32> for &Size {
    fn into(self) -> u32 {
        match self {
            Size::Sm => 640,
            Size::Md => 768,
            Size::Lg => 1024,
            Size::Pixels(px) => *px,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Sm,
    Md,
    Lg,
}

impl ImageSize {
    pub fn resolution(&self) -> &'static str {
        match self {
            ImageSize::Sm => "256x256",
            ImageSize::Md => "512x512",
            ImageSize::Lg => "1024x1024",
        }
    }
}
