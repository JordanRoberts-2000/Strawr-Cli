#[derive(thiserror::Error, Debug)]
pub enum ClipboardError {
    #[error("failed to access clipboard")]
    ClipboardAccess(arboard::Error),
    #[error("failed to set text to clipboard")]
    ClipboardSet(arboard::Error),
}
