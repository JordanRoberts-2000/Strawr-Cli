use crate::{error::IoError, utils::editor::EditorError};

#[derive(thiserror::Error, Debug)]
pub enum TempError {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("Editor failed to open")]
    EditorFailed(#[from] EditorError),
}
