use super::{traits::Clipboard, ClipboardError, ClipboardRepo};

pub struct ClipboardService {
    pub(super) repo: Box<dyn Clipboard>,
}

impl ClipboardService {
    pub fn new() -> Self {
        Self {
            repo: Box::new(ClipboardRepo),
        }
    }

    pub fn set_repo(&mut self, repo: impl Clipboard + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }

    pub fn save_to_clipboard(&self, text: &str) -> Result<(), ClipboardError> {
        self.repo.set_text(text)?;
        Ok(())
    }
}
