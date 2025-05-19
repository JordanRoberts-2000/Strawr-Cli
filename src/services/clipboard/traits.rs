use super::error::ClipboardError;

pub trait Clipboard {
    fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
}
