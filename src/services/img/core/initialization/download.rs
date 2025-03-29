use std::env;

use image::{guess_format, GenericImageView};
use reqwest::blocking;
use url::Url;

use crate::services::img::{
    core::ImgSrc,
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn download(url: &String) -> Result<Self> {
        let parsed_url = Url::parse(url).map_err(|e| ImgError::InvalidUrl {
            url: url.clone(),
            source: e,
        })?;

        let response = blocking::get(parsed_url.clone()).map_err(|e| {
            ImgError::Custom(format!(
                "Failed to download image from '{}': {}",
                parsed_url, e
            ))
        })?;

        let bytes = response.bytes().map_err(|e| {
            ImgError::Custom(format!(
                "Failed to read bytes from response for '{}': {}",
                parsed_url, e
            ))
        })?;

        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;
        let size_bytes = bytes.len();

        let img = image::load_from_memory(&bytes).map_err(|e| ImgError::Decoding {
            id: url.clone(),
            source: e,
            format,
        })?;

        let (width, height) = img.dimensions();

        let cwd = env::current_dir().map_err(|e| ImgError::Io {
            context: "Failed to get current working directory".into(),
            source: e,
        })?;

        let ext = match format.extensions_str().first() {
            Some(ext) => ext,
            None => return Err(ImgError::Custom("No valid extentions found".to_string())),
        };
        let filename = format!("image.{}", ext);
        let target = cwd.join(filename);

        Ok(Self {
            img,
            src: ImgSrc::Remote {
                url: parsed_url,
                target,
            },
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
            size_bytes,
        })
    }
}
