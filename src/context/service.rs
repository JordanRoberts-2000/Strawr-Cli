use std::path::Path;

use crate::services::{
    editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
    errors::EditorLauncherError,
    prompt::{traits::CliInput, UserInput},
};

pub struct CliService {
    pub prompt: Box<dyn CliInput>,
    // pub clipboard: Box<dyn Clipboard>,
    // pub keychain: Box<dyn Keychain>,
    pub editor_launcher: Box<dyn EditorLauncher>,
}

impl CliService {
    pub fn new() -> Self {
        Self {
            prompt: Box::new(UserInput),
            editor_launcher: Box::new(CliEditorLauncher),
        }
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.editor_launcher.open(editor, &path)?;

        Ok(())
    }
}
