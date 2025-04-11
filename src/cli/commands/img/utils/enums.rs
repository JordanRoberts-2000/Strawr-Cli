use image::ImageFormat;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    #[serde(alias = "jpg")]
    Jpeg,
    WebP,
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
            ValidImageFormat::WebP => Ok(ImageFormat::WebP),
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
            ValidImageFormat::WebP => Ok(ImageFormat::WebP),
            ValidImageFormat::Original => Err(ImageFormatConversionError::NoConcreteFormat),
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
