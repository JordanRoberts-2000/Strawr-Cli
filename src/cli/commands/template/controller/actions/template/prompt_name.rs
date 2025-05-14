use crate::template::{types::ValidTemplateName, TemplateController, TemplateError};

impl TemplateController {
    pub fn prompt_template_name(&self) -> Result<ValidTemplateName, TemplateError> {
        let input = self.view.enter_template_name()?;
        let template: ValidTemplateName = input.parse().map_err(TemplateError::InvalidTemplate)?;
        Ok(template)
    }
}
