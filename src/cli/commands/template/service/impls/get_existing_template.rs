use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn get_existing_template(&self, str: &str) -> Result<Template, TemplateError> {
        let valid_name = Template::validate_name(str)?;
        let template = Template::new(&valid_name, &self.templates_path);

        if !template.path.exists() {
            return Err(TemplateError::TemplateNotFound(template.name.clone()));
        }

        Ok(template)
    }
}
