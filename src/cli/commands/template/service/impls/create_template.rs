use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn create_template(&self, str: &str) -> Result<Template, TemplateError> {
        let valid_name = Template::validate_name(str)?;
        let template = Template::new(&valid_name, &self.templates_path);

        if template.path.exists() {
            return Err(TemplateError::TemplateAlreadyExists(
                template.name.to_string(),
            ));
        }

        self.fs.create_dir_all(&template.default_path())?;

        Ok(template)
    }
}
