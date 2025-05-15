use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn ensure_template_exists(&self, template: &Template) -> Result<(), TemplateError> {
        if !template.path.exists() {
            return Err(TemplateError::TemplateNotFound(template.id.to_string()));
        }

        Ok(())
    }
}
