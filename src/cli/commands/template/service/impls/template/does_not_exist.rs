use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn ensure_template_does_not_exist(&self, template: &Template) -> Result<(), TemplateError> {
        if template.path.exists() {
            return Err(TemplateError::TemplateAlreadyExists(
                template.id.to_string(),
            ));
        }

        Ok(())
    }
}
