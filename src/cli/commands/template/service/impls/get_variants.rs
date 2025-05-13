use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn get_variants(&self, template: &Template) -> Result<Vec<String>, TemplateError> {
        let variants = self.fs.sub_dirs(&template.path)?;
        Ok(variants)
    }
}
