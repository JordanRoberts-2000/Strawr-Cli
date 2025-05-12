use crate::template::{service::TemplateService, TemplateError};

impl TemplateService {
    pub fn has_templates(&self) -> Result<bool, TemplateError> {
        let has_templates = self.fs.empty_dir(&self.templates_path)?;
        Ok(has_templates)
    }
}
