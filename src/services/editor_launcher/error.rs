#[derive(thiserror::Error, Debug)]
pub enum EditorLauncherError {
    #[error("editor exited with non-zero status: {0:?}")]
    NonZeroExit(Option<i32>), // Option because the status code might not exist on all platforms

    #[error("failed to launch editor: {0}")]
    LaunchFailed(#[from] std::io::Error),

    #[error("editor command not found in PATH: {0}")]
    NotFound(String),

    #[error("Path provided does not exist '{}'", .0.display())]
    PathDoesntExist(std::path::PathBuf),
}
