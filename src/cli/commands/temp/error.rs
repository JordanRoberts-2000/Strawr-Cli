use crate::utils::editor::EditorError;

#[derive(thiserror::Error, Debug)]
pub enum TempError {
    #[error("failed to create temporary directory: {0}")]
    TempDirCreation(std::io::Error),
    #[error("Editor failed to open")]
    EditorFailed(#[from] EditorError),
}
