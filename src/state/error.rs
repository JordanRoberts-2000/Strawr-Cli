#[derive(thiserror::Error, Debug)]
pub enum StateError {
    #[error("io error: {source}")]
    Io {
        context: String,
        source: std::io::Error,
    },
    #[error("Home directory could not be found. Please set the {env_var} environment variable.")]
    HomeDirNotFound { env_var: &'static str },
}
