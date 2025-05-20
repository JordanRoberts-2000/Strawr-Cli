use std::fs;

use crate::{
    error::IoError,
    template::{service::TemplateService, TemplateError},
};

impl<'svc> TemplateService<'svc> {
    pub fn init_templates_folder(&self) -> Result<(), TemplateError> {
        if !self.templates_path.is_dir() {
            fs::create_dir_all(&self.templates_path)
                .map_err(|e| IoError::CreateDir(e, self.templates_path.to_path_buf()))?;
        }

        Ok(())
    }
}
