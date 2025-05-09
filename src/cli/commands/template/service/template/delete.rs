use std::fs::remove_dir_all;

use crate::{
    error::IoError,
    template::{utils::Template, TemplateError, TemplateService},
};

impl<'a> TemplateService<'a> {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        let msg = format!(
            "Are you sure you want to delete template '{}'",
            template.name
        );
        if !self.confirm(&msg)? {
            return Ok(());
        }

        remove_dir_all(&template.path).map_err(|e| IoError::DeleteDir(e, template.path.clone()))?;

        Ok(())
    }
}
