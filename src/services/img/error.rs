use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImgError>;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error(
        "failed to convert img '{:?}' to format '{:?}', err:{}",
        path,
        format,
        err_string
    )]
    Conversion {
        err_string: String,
        path: PathBuf,
        format: image::ImageFormat,
    },
    #[error(
        "failed to decode image '{:?}' after to format '{:?}', err: {}",
        path,
        format,
        source
    )]
    ImageDecoding {
        path: PathBuf,
        source: image::ImageError,
        format: image::ImageFormat,
    },
    #[error("failed to open img '{:?}', err:{}", path, source)]
    Open {
        source: image::ImageError,
        path: PathBuf,
    },
    #[error("{}, err: {}", context, source)]
    Io {
        context: String,
        source: std::io::Error,
    },
    #[error("failed to retrieve file format")]
    GuessFormat,
    #[error("failed to create new image '{:?}', err: {}", output, source)]
    Save {
        source: image::ImageError,
        output: PathBuf,
    },
}
