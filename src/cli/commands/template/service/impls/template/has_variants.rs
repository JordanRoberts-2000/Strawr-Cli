use crate::template::{models::Template, service::TemplateService, TemplateError};

impl<'svc> TemplateService<'svc> {
    pub(crate) fn has_variants(&self, template: &Template) -> Result<bool, TemplateError> {
        let has_variants = self.fs.min_entries(&template.path, 2)?;
        Ok(has_variants)
    }
}
