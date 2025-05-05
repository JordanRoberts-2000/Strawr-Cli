use std::path::PathBuf;

use crate::cli::commands::template::{
    command::manager::TemplateManager, TemplateError, DEFAULT_FOLDER,
};

impl<'a> TemplateManager<'a> {
    pub fn open_template(
        &self,
        path: &PathBuf,
        variant: &Option<&str>,
    ) -> Result<(), TemplateError> {
        let mut path = path.clone();

        match variant {
            Some(v) => path = path.join(v),
            None => path = path.join(DEFAULT_FOLDER),
        };

        self.ctx.editor.open(self.editor, &path)?;

        Ok(())
    }
}
