use std::path::Path;

use crate::{
    template::{TemplateController, TemplateError},
    utils,
};

impl<'c> TemplateController<'c> {
    pub fn inject_template_files(&self, path: &Path, output: &Path) -> Result<(), TemplateError> {
        if !utils::fs::dir_empty(&output)? && !self.view.output_not_empty_warning()? {
            return Ok(());
        }

        utils::fs::copy_dir_contents(&path, &output)?;
        Ok(())
    }
}
