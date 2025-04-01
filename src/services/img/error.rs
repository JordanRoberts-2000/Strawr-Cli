use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, ImgError>;

#[derive(thiserror::Error, Debug)]
pub enum ImgError {
    #[error("failed to convert img '{id}' to format '{format:?}'")]
    Conversion {
        source: image::ImageError,
        id: String,
        format: image::ImageFormat,
    },

    #[error("failed to convert img '{id}' to format 'webp'")]
    WebPConversion {
        err: webp::WebPEncodingError,
        id: String,
    },

    #[error(
        "failed to decode image '{}' to format '{:?}', err: {}",
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

    #[error("Failed to download image from '{url}': {source}")]
    DownloadFailed { url: String, source: reqwest::Error },

    #[error("Failed to read bytes from response for '{url}': {source}")]
    ResponseReadFailed { url: String, source: reqwest::Error },

    #[error("failed to retrieve file format")]
    GuessFormat,

    #[error("No valid extentions found")]
    ExtensionInvalid,

    #[error("failed to create new image {:?}, err: {}", output, source)]
    Save {
        source: image::ImageError,
        output: PathBuf,
    },

    #[error("Could not retrieve palette: {0}")]
    GetColors(color_thief::Error),

    #[error("No colors in retrieved palette")]
    EmptyPalette,

    #[error("Blurhash failed to encode img, err: {0}")]
    BlurHash(blurhash::Error),

    #[error("Output path does not have a file name: {0:?}")]
    MissingFileName(PathBuf),

    #[error("failed to parse url '{url}', {source}")]
    UrlParseFailed {
        url: String,
        source: url::ParseError,
    },
}
