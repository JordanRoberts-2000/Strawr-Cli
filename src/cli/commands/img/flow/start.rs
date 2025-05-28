use std::{marker::PhantomData, path::PathBuf};

use colored::Colorize;
use walkdir::WalkDir;

use crate::{
    commands::img::{enums::ImageInput, ImgCmdError},
    img::Img,
    validation::validate,
};

use super::{
    core::{ImagesResolved, Start},
    ImgFlow,
};

impl ImgFlow<Start> {
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            images: Vec::new(),
        }
    }

    pub fn resolve_images(
        self,
        input: &ImageInput,
    ) -> Result<ImgFlow<ImagesResolved>, ImgCmdError> {
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

    fn handle_dir(input: &PathBuf) -> Result<Vec<Img>, ImgCmdError> {
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
            return Err(ImgCmdError::NoImagesFilesFound(input.clone()));
        }

        Ok(images)
    }
}
