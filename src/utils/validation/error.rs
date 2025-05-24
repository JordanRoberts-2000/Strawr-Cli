use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("path is not a file: {0}")]
    NotAFile(PathBuf),

    #[error("`{0}` is a reserved value")]
    Reserved(String),

    #[error("cannot be empty")]
    Empty,

    #[error("max size exceeded, max size {max}: {input})")]
    TooLong { max: usize, input: String },

    #[error("cannot be turned into a slug (result was empty): {0}")]
    EmptySlug(String),

    #[error("cannot be turned into a slug (max size of {max} exceeded), slug: {slug}")]
    SlugTooLong { max: usize, slug: String },

    #[error("`{0}` is not a valid URL")]
    InvalidUrl(String),

    #[error("unsupported URL scheme `{scheme}` in `{input}`; only http/https allowed")]
    UnsupportedScheme { input: String, scheme: String },

    #[error("URL `{0}` must include a host, e.g. `https://example.com`")]
    MissingHost(String),
}
