use std::fs;

use crate::{
    cli::commands::template::{
        command::{service::TemplateService, utils::Template},
        TemplateError,
    },
    error::IoError,
};

impl<'a> TemplateService<'a> {
    pub fn has_variants(&self, template: &Template) -> Result<bool, TemplateError> {
        let mut entries =
            fs::read_dir(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;
        let mut count = 0;

        while let Some(_) = entries.next() {
            count += 1;
            if count >= 2 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
