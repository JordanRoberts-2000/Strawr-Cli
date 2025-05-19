use std::{
    cell::OnceCell,
    path::{Path, PathBuf},
};

use crate::{
    services::{
        editor_launcher::{traits::EditorLauncher, CliEditorLauncher, Editor},
        errors::EditorLauncherError,
        fs::{CliFsRepository, FsRepository},
    },
    template::constants::TEMPLATES_FOLDER_NAME,
    CliContext,
};

pub struct TemplateService {
    pub templates_path: PathBuf,
    pub editor_launcher: OnceCell<Box<dyn EditorLauncher>>,
    pub fs: Box<dyn FsRepository>,
}

impl TemplateService {
    pub fn new(ctx: &CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        Self {
            templates_path,
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
