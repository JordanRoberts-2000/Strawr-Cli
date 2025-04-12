use std::env;

use image::{guess_format, GenericImageView};
use uuid::Uuid;

use crate::services::img::{
    core::ImgSrc,
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;
        let size_bytes = bytes.len();

        let img = image::load_from_memory(&bytes).map_err(|e| ImgError::Decoding {
            id: id.clone(),
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
            None => return Err(ImgError::ExtensionInvalid),
        };
        let file_name = format!("{}.{}", id, ext);
        let target = cwd.join(&file_name);

        Ok(Self {
            img,
            src: ImgSrc::Bytes { id: id.clone() },
            target_path: target,
            file_name,
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
            size_bytes,
        })
    }
}
