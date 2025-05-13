use crate::template::{service::TemplateService, TemplateError};

impl TemplateService {
    pub fn has_templates(&self) -> Result<bool, TemplateError> {
        let has_templates = self.fs.dir_empty(&self.templates_path)?;
        Ok(has_templates)
    }
}
