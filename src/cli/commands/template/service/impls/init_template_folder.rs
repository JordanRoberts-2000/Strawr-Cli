use crate::template::{service::TemplateService, TemplateError};

impl TemplateService {
    pub fn init_templates_folder(&self) -> Result<(), TemplateError> {
        if !self.templates_path.is_dir() {
            self.fs.create_dir_all(&self.templates_path)?;
        }

        Ok(())
    }
}
