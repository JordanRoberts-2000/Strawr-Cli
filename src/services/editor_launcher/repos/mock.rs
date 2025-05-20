use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};

use crate::services::editor_launcher::{traits::EditorLauncher, Editor, EditorLauncherError};

pub struct MockEditorLauncherRepo {
    history: RefCell<Vec<(Editor, PathBuf)>>,
}

impl EditorLauncher for MockEditorLauncherRepo {
    fn open(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.history
            .borrow_mut()
            .push((editor.clone(), path.to_path_buf()));
        Ok(())
    }
}

impl MockEditorLauncherRepo {
    pub fn last(&self) -> Option<(Editor, PathBuf)> {
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

    // Helper to construct a fresh mock
    fn make_mock() -> MockEditorLauncherRepo {
        MockEditorLauncherRepo {
            history: RefCell::new(Vec::new()),
        }
    }

    #[test]
    fn open_records_calls_and_path() {
        let repo = make_mock();
        // initially empty
        assert_eq!(repo.call_count(), 0);
        assert!(repo.last().is_none());

        let editor = Editor::VsCode;
        let path1 = PathBuf::from("/tmp/file1");
        repo.open(&editor, &path1).unwrap();

        // one call, last() returns exactly that tuple
        assert_eq!(repo.call_count(), 1);
        assert_eq!(repo.last(), Some((editor.clone(), path1.clone())));

        // a second call with a different path
        let path2 = PathBuf::from("/var/log/app.log");
        repo.open(&editor, &path2).unwrap();
        assert_eq!(repo.call_count(), 2);
        assert_eq!(repo.last(), Some((editor.clone(), path2.clone())));
    }
}
