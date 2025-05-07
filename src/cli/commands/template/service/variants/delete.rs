use std::fs::remove_dir_all;

use crate::{
    cli::commands::template::{service::TemplateService, utils::Variant, TemplateError},
    error::IoError,
};

impl<'a> TemplateService<'a> {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        let msg = format!("Are you sure you want to delete variant '{}'", variant.name);
        if !self.confirm(&msg)? {
            return Ok(());
        }

        remove_dir_all(&variant.path).map_err(|e| IoError::DeleteDir(e, variant.path.clone()))?;

        Ok(())
    }
}
