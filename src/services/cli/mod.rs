use std::{cell::OnceCell, path::Path};

use crate::services::{
    editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
    errors::EditorLauncherError,
    prompt::{user::UserInputRepo, PromptService},
};

pub struct CliService {
    pub prompt: OnceCell<PromptService>,
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

    pub fn prompt(&self) -> &PromptService {
        self.prompt
            .get_or_init(|| PromptService::new(UserInputRepo))
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.editor_launcher
            .get_or_init(|| Box::new(CliEditorLauncher))
            .open(editor, &path)?;

        Ok(())
    }
}
