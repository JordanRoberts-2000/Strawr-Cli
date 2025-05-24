use super::error::ClipboardError;

pub trait ClipboardRepo {
    fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
}
