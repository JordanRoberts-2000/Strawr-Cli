use std::path::Path;

use url::Url;

use crate::cli::commands::img::{ImgCommand, ImgError};

#[derive(Debug, PartialEq)]
pub enum InputType {
    Url,
    File,
    Directory,
}

impl ImgCommand {
    pub fn detect_input_type(&self, input: &String) -> Result<InputType, ImgError> {
        if let Ok(url) = Url::parse(input) {
            if url.has_host() {
                return Ok(InputType::Url);
            }
        }

        let path = Path::new(input);

        if !path.exists() {
            return Err(ImgError::InputNotFound(input.clone()));
        }

        if path.is_dir() {
            Ok(InputType::Directory)
        } else if path.is_file() {
            Ok(InputType::File)
        } else {
            Err(ImgError::UnknownInputType)
        }
    }
}
