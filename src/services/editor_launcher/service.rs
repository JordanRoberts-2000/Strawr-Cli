use std::path::Path;

use super::{traits::EditorLauncher, Editor, EditorLauncherError, EditorLauncherRepo};

pub struct EditorLauncherService {
    pub(super) repo: Box<dyn EditorLauncher>,
}

impl EditorLauncherService {
    pub fn new() -> Self {
        Self {
            repo: Box::new(EditorLauncherRepo),
        }
    }

    pub fn set_repo(&mut self, repo: impl EditorLauncher + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }

    pub fn launch_editor(
        &self,
        editor: &Editor,
        path: impl AsRef<Path>,
    ) -> Result<(), EditorLauncherError> {
        self.repo.open(editor, path.as_ref())?;
        Ok(())
    }
}
