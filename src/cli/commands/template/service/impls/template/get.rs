use crate::template::{service::TemplateService, TemplateError};

impl TemplateService {
    pub fn get_templates(&self) -> Result<Vec<String>, TemplateError> {
        let templates = self.fs.sub_dirs(&self.templates_path)?;
        Ok(templates)
    }
}
