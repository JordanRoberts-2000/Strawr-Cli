use std::{
    cell::OnceCell,
    path::{Path, PathBuf},
};

use crate::services::{
    editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
    errors::EditorLauncherError,
    fs::{CliFsRepository, FsRepository},
};

pub struct TemplateService {
    pub templates_path: PathBuf,
    pub editor_launcher: OnceCell<Box<dyn EditorLauncher>>,
    pub fs: Box<dyn FsRepository>,
}

impl TemplateService {
    pub fn new(templates_path: &PathBuf) -> Self {
        Self {
            templates_path: templates_path.clone(),
            editor_launcher: OnceCell::new(),
            fs: Box::new(CliFsRepository),
        }
    }

    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        self.editor_launcher
            .get_or_init(|| Box::new(CliEditorLauncher))
            .open(editor, &path)?;

        Ok(())
    }
}
