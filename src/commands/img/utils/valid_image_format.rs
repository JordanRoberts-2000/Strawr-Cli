use crate::error::{Error, Result};
use image::ImageFormat;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    Avif,
    Webp,
    Jpeg,
    Png,
    Original,
}

impl ValidImageFormat {
    pub fn extension(&self) -> Result<&'static str> {
        match self {
            ValidImageFormat::Avif => Ok("avif"),
            ValidImageFormat::Webp => Ok("webp"),
            ValidImageFormat::Jpeg => Ok("jpg"),
            ValidImageFormat::Png => Ok("png"),
            ValidImageFormat::Original => Err(Error::Custom("Should not occure".into())),
        }
    }
}

impl TryFrom<ValidImageFormat> for ImageFormat {
    type Error = Error;

    fn try_from(fmt: ValidImageFormat) -> Result<Self> {
        match fmt {
            ValidImageFormat::Avif => Ok(ImageFormat::Avif),
            ValidImageFormat::Webp => Ok(ImageFormat::WebP),
            ValidImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ValidImageFormat::Png => Ok(ImageFormat::Png),
            ValidImageFormat::Original => Err(Error::Custom(
                "Original format indicates no conversion should be applied".to_string(),
            )),
        }
    }
}
