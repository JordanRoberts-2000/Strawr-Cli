use crate::template::{
    models::Template, types::ValidTemplateName, TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn prompt_template_name(&self) -> Result<Template, TemplateError> {
        let input = self.view.enter_template_name()?;
        let valid_template_name: ValidTemplateName =
            input.parse().map_err(TemplateError::InvalidTemplate)?;

        Ok(Template::new(
            &valid_template_name,
            &self.service.templates_path,
        ))
    }
}
