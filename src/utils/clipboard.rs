use arboard::Clipboard;

use crate::error::{Error, Result};

pub fn to_clipboard(text: &String) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::Custom(format!("Failed to access clipboard: {}", e)))?;
    clipboard
        .set_text(text)
        .map_err(|e| Error::Custom(format!("Failed to set text to clipboard: {}", e)))?;

    log::trace!("Added text to clipboard");
    Ok(())
}
