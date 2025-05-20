use crate::template::{models::Template, service::TemplateService, TemplateError};

impl<'svc> TemplateService<'svc> {
    pub fn get_variants(&self, template: &Template) -> Result<Vec<String>, TemplateError> {
        let variants = self.fs.sub_dirs(&template.path)?;
        Ok(variants)
    }
}
