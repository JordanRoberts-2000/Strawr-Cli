use std::path::Path;

use crate::services::editor_launcher::{traits::EditorLauncher, Editor, EditorLauncherError};

pub struct TestEditorLauncher {
    should_fail: bool,
}

impl TestEditorLauncher {
    pub fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

impl EditorLauncher for TestEditorLauncher {
    fn open(&self, _editor: &Editor, _path: &Path) -> Result<(), EditorLauncherError> {
        if self.should_fail {
            Err(EditorLauncherError::LaunchFailed(std::io::Error::new(
                std::io::ErrorKind::Other,
                "simulated failure",
            )))
        } else {
            Ok(())
        }
    }
}
