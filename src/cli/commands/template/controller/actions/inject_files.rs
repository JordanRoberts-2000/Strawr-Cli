use std::path::Path;

use crate::template::{TemplateController, TemplateError};

impl TemplateController {
    pub fn inject_template_files(&self, path: &Path, output: &Path) -> Result<(), TemplateError> {
        if !self.service.fs.dir_empty(&output)? && self.view.output_not_empty_warning()? {
            return Ok(());
        }

        self.service.fs.copy_dir_contents(&path, &output)?;
        Ok(())
    }
}
