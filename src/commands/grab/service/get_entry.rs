use crate::error::{Error, Result};
use arboard::Clipboard;

use super::GrabService;

impl GrabService {
    pub fn get_entry(&self, val: &String) -> Result<()> {
        log::debug!("Value '{}' copied to clipboard", val);

        let mut clipboard = Clipboard::new()
            .map_err(|e| Error::Custom(format!("Failed to access clipboard, {}", e)))?;
        clipboard
            .set_text(val)
            .map_err(|e| Error::Custom(format!("Failed to save text to clipboard, {}", e)))?;

        Ok(())
    }
}
