use std::cell::RefCell;

use crate::services::clipboard::{ClipboardError, ClipboardRepo};

pub struct MockClipboardRepo {
    history: RefCell<Vec<String>>,
}

impl ClipboardRepo for MockClipboardRepo {
    fn set_text(&self, text: &str) -> Result<(), ClipboardError> {
        self.history.borrow_mut().push(text.to_string());
        Ok(())
    }
}

impl MockClipboardRepo {
    pub fn last(&self) -> Option<String> {
        self.history.borrow().last().cloned()
    }

    pub fn call_count(&self) -> usize {
        self.history.borrow().len()
    }

    pub fn print_history(&self) {
        println!("{:#?}", self.history);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_clipboard_records_calls() {
        let mock = MockClipboardRepo {
            history: RefCell::new(Vec::new()),
        };

        // Nothing called yet:
        assert_eq!(mock.call_count(), 0);
        assert_eq!(mock.last(), None);

        // Call set_text once:
        mock.set_text("foo").unwrap();
        assert_eq!(mock.call_count(), 1);
        assert_eq!(mock.last().as_deref(), Some("foo"));

        // Call set_text a second time:
        mock.set_text("bar").unwrap();
        assert_eq!(mock.call_count(), 2);
        assert_eq!(mock.last().as_deref(), Some("bar"));
    }
}
