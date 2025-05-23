use {
    std::{path::PathBuf, str::FromStr},
    url::Url,
};

use crate::{
    commands::img::ImgError,
    utils::validation::{validate, ValidationError},
};

#[derive(Debug, PartialEq, Clone)]
pub enum ImageInput {
    Url(Url),
    File(PathBuf),
    Directory(PathBuf),
}

impl FromStr for ImageInput {
    type Err = ImgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        match validate::remote_url(input) {
            Ok(url) => return Ok(ImageInput::Url(url)),
            Err(
                ValidationError::MissingHost(_)
                | ValidationError::UnsupportedScheme {
                    input: _,
                    scheme: _,
                },
            ) => {
                return Err(ImgError::InputUrlNotRemote(s.to_string()));
            }
            Err(_) => {}
        }

        match validate::existing_file(input) {
            Ok(path) => return Ok(ImageInput::File(path)),
            Err(ValidationError::PathNotFound(p)) => {
                return Err(ImgError::InputNotFound(p));
            }
            Err(_) => {}
        }

        match validate::existing_dir(input) {
            Ok(path) => return Ok(ImageInput::Directory(path)),
            Err(ValidationError::PathNotFound(p)) => {
                return Err(ImgError::InputNotFound(p));
            }
            Err(_) => {}
        }

        Err(ImgError::UnknownInputType(input.to_string()))
    }
}
