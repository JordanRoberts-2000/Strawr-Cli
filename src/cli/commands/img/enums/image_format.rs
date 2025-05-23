use image::ImageFormat;

use crate::commands::img::ImgError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    #[serde(alias = "jpg")]
    #[clap(alias = "jpg")]
    Jpeg,
    Webp,
    Png,
    Original,
}

impl TryFrom<ValidImageFormat> for ImageFormat {
    type Error = ImgError;

    fn try_from(value: ValidImageFormat) -> Result<Self, Self::Error> {
        match value {
            ValidImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ValidImageFormat::Png => Ok(ImageFormat::Png),
            ValidImageFormat::Webp => Ok(ImageFormat::WebP),
            ValidImageFormat::Original => Err(ImgError::NoConcreteFormat),
        }
    }
}

impl TryFrom<&ValidImageFormat> for ImageFormat {
    type Error = ImgError;

    fn try_from(value: &ValidImageFormat) -> Result<Self, Self::Error> {
        match value {
            ValidImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ValidImageFormat::Png => Ok(ImageFormat::Png),
            ValidImageFormat::Webp => Ok(ImageFormat::WebP),
            ValidImageFormat::Original => Err(ImgError::NoConcreteFormat),
        }
    }
}
