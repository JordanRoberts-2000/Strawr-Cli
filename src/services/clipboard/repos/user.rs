use arboard::Clipboard as ArboardClipboard;

use crate::services::clipboard::{ClipboardError, ClipboardRepo};

pub struct UserClipboardRepo;

impl ClipboardRepo for UserClipboardRepo {
    fn set_text(&self, text: &str) -> Result<(), ClipboardError> {
        let mut clipboard = ArboardClipboard::new().map_err(ClipboardError::ClipboardAccess)?;
        clipboard
            .set_text(text)
            .map_err(ClipboardError::ClipboardSet)?;
        Ok(())
    }
}
