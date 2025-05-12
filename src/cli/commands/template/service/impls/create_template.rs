use crate::template::{
    constants::DEFAULT_FOLDER, models::Template, service::TemplateService, TemplateError,
};

impl TemplateService {
    pub fn create_template(&self, str: &str) -> Result<Template, TemplateError> {
        let template = Template::validate_name(str)?;
        let template_path = self.templates_path.join(&template);

        if template_path.exists() {
            return Err(TemplateError::Validation(format!(
                "Template '{}' already exists",
                template
            )));
        }

        let default_path = template_path.join(DEFAULT_FOLDER);

        self.fs.create_dir_all(&default_path)?;

        Ok(Template::new(&template, &template_path))
    }
}
