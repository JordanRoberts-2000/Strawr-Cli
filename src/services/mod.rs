pub(crate) mod editor_launcher;
pub mod prompt;
pub(crate) mod storage;

pub mod errors {
    pub use super::editor_launcher::error::EditorLauncherError;
    pub use super::prompt::PromptError;
    pub use super::storage::StorageError;
}
