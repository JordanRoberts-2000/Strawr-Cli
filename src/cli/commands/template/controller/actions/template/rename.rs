use crate::template::{
    models::Template, types::ValidTemplateName, TemplateController, TemplateError,
};

impl TemplateController {
    pub fn rename_template(&self, template: &Template) -> Result<(), TemplateError> {
        let input = self.view.enter_template_name()?;
        let new_name: ValidTemplateName = input.parse().map_err(TemplateError::InvalidTemplate)?;

        self.service.rename_template(&template, &new_name)?;
        self.view.template_renamed(&template, &new_name);
        Ok(())
    }
}
