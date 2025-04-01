use arboard::Clipboard;

pub fn clipboard(text: &String) -> Result<(), ClipboardError> {
    let mut clipboard = Clipboard::new().map_err(ClipboardError::ClipboardAccess)?;
    clipboard
        .set_text(text)
        .map_err(ClipboardError::ClipboardSet)?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ClipboardError {
    #[error("failed to access clipboard")]
    ClipboardAccess(arboard::Error),
    #[error("failed to set text to clipboard")]
    ClipboardSet(arboard::Error),
}
