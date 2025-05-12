use crate::{
    error::IoError,
    template::{TemplateError, TemplateManager},
    utils::fs::is_dir_empty,
};

impl<'a> TemplateManager<'a> {
    pub fn has_templates(&self) -> Result<bool, TemplateError> {
        let is_empty = is_dir_empty(self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.to_path_buf()))?;

        Ok(!is_empty)
    }
}
