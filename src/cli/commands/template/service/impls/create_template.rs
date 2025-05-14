use crate::template::{
    models::Template, service::TemplateService, types::ValidTemplateName, TemplateError,
};

impl TemplateService {
    pub fn create_template(&self, template: &ValidTemplateName) -> Result<Template, TemplateError> {
        let template = Template::new(template, &self.templates_path);

        if template.path.exists() {
            return Err(TemplateError::TemplateAlreadyExists(
                template.name.to_string(),
            ));
        }

        self.fs.create_dir_all(&template.default_path())?;

        Ok(template)
    }
}
