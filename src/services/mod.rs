pub(crate) mod cli;
pub mod clipboard;
pub mod editor_launcher;
pub mod prompt;
pub(crate) mod storage;

pub mod errors {
    pub use super::clipboard::ClipboardError;
    pub use super::editor_launcher::EditorLauncherError;
    pub use super::prompt::PromptServiceError;
    pub use super::storage::StorageError;
}
