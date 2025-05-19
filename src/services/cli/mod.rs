use std::{cell::OnceCell, path::Path};

use crate::{
    services::{
        editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
        errors::EditorLauncherError,
        prompt::{PasswordDisplay, PromptService},
    },
    CliConfig,
};

pub struct CliService {
    password_input_display_mode: PasswordDisplay,
    pub prompt: OnceCell<PromptService>,
    // pub clipboard: Box<dyn Clipboard>,
    // pub keychain: Box<dyn Keychain>,
    pub editor_launcher: OnceCell<Box<dyn EditorLauncher>>,
}

impl CliService {
    pub fn new(config: &CliConfig) -> Self {
        Self {
            password_input_display_mode: config.password_input_display_mode,
            prompt: OnceCell::new(),
            editor_launcher: OnceCell::new(),
        }
    }

    pub fn prompt(&self) -> &PromptService {
        self.prompt.get_or_init(|| {
            let mut service = PromptService::new();
            service.set_password_mode(&self.password_input_display_mode.into());
            service
        })
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.editor_launcher
            .get_or_init(|| Box::new(CliEditorLauncher))
            .open(editor, &path)?;

        Ok(())
    }
}
