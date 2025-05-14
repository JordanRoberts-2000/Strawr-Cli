use crate::template::{
    models::Template, service::TemplateService, types::ValidTemplateName, TemplateError,
};

impl TemplateService {
    pub fn get_existing_template(
        &self,
        template: &ValidTemplateName,
    ) -> Result<Template, TemplateError> {
        let template = Template::new(&template, &self.templates_path);

        if !template.path.exists() {
            return Err(TemplateError::TemplateNotFound(template.name.to_string()));
        }

        Ok(template)
    }
}
