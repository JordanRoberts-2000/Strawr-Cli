use std::path::Path;

use crate::{
    services::prompt::{CliInput, UserInput},
    template::TemplateError,
    utils::{
        editor::{CliEditor, EditorLauncher},
        Editor,
    },
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
            editor_launcher: Box::new(CliEditor),
        }
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), TemplateError> {
        self.editor_launcher.open(editor, &path)?;

        Ok(())
    }
}
