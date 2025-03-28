use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImgError>;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error(
        "failed to convert img '{}' to format '{:?}', err: {}",
        id,
        format,
        err_string
    )]
    Conversion {
        err_string: String,
        id: String,
        format: image::ImageFormat,
    },
    #[error(
        "failed to decode image '{}' after to format '{:?}', err: {}",
        id,
        format,
        source
    )]
    Decoding {
        id: String,
        source: image::ImageError,
        format: image::ImageFormat,
    },
    #[error("failed to open img '{:?}', err: {}", path, source)]
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
    #[error("{}, err: {}", source, context)]
    Color {
        source: color_thief::Error,
        context: String,
    },
    #[error("Blurhash failed to encode img, err: {0}")]
    BlurHash(blurhash::Error),
    #[error("Output path does not have a file name: {0:?}")]
    MissingFileName(PathBuf),
    #[error("failed to parse url '{url}', {source}")]
    InvalidUrl {
        url: String,
        source: url::ParseError,
    },
    #[error("{0}")]
    Custom(String),
}
