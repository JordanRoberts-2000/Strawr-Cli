use std::{cell::OnceCell, path::Path};

use crate::services::{
    editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
    errors::EditorLauncherError,
    prompt::{traits::CliInput, UserInput},
};

pub struct CliService<I: CliInput> {
    pub prompt: I,
    // pub clipboard: Box<dyn Clipboard>,
    // pub keychain: Box<dyn Keychain>,
    pub editor_launcher: OnceCell<Box<dyn EditorLauncher>>,
}

impl CliService {
    pub fn new() -> Self {
        Self {
            prompt: OnceCell::new(),
            editor_launcher: OnceCell::new(),
        }
    }

    pub fn prompt(&self) -> &dyn CliInput {
        self.prompt.get_or_init(|| Box::new(UserInput)).as_ref()
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.editor_launcher
            .get_or_init(|| Box::new(CliEditorLauncher))
            .open(editor, &path)?;

        Ok(())
    }
}
