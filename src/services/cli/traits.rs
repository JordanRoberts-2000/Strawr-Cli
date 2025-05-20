use std::path::Path;

use crate::{
    services::{editor_launcher::Editor, errors::EditorLauncherError, prompt::PromptService},
    CliService,
};

pub trait HasEditorLauncherService {
    fn cli(&self) -> &CliService;

    fn launch_editor(
        &self,
        editor: &Editor,
        path: impl AsRef<Path>,
    ) -> Result<(), EditorLauncherError> {
        self.cli()
            .init_launch_editor()
            .launch_editor(editor, path.as_ref())
    }
}

pub trait HasPromptService {
    fn cli(&self) -> &CliService;

    fn prompt(&self) -> &PromptService {
        self.cli().init_prompt()
    }
}
