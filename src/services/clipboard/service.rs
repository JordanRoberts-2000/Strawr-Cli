use super::{traits::ClipboardRepo, ClipboardError, UserClipboardRepo};

pub struct ClipboardService {
    pub(super) repo: Box<dyn ClipboardRepo>,
}

impl ClipboardService {
    pub fn new() -> Self {
        Self {
            repo: Box::new(UserClipboardRepo),
        }
    }

    pub fn set_repo(&mut self, repo: impl ClipboardRepo + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }

    pub fn save_to_clipboard(&self, text: &str) -> Result<(), ClipboardError> {
        self.repo.set_text(text)?;
        Ok(())
    }
}
