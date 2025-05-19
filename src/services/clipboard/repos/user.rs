use arboard::Clipboard as ArboardClipboard;

use crate::services::clipboard::{Clipboard, ClipboardError};

pub struct ClipboardRepo;

impl Clipboard for ClipboardRepo {
    fn set_text(&self, text: &str) -> Result<(), ClipboardError> {
        let mut clipboard = ArboardClipboard::new().map_err(ClipboardError::ClipboardAccess)?;
        clipboard
            .set_text(text)
            .map_err(ClipboardError::ClipboardSet)?;
        Ok(())
    }
}
