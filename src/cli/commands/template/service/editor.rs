use std::path::Path;

use crate::{cli::commands::template::TemplateError, utils::Editor};

use super::TemplateService;

impl<'a> TemplateService<'a> {
    pub fn launch_editor(&self, editor: &Editor, path: &Path) -> Result<(), TemplateError> {
        self.editor_launcher.open(editor, &path)?;

        Ok(())
    }
}
