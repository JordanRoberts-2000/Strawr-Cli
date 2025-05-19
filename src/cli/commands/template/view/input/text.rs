use crate::template::{TemplateError, TemplateView};

impl<'a> TemplateView<'a> {
    pub fn enter_template_name(&self) -> Result<String, TemplateError> {
        let msg = "Enter template name:";
        let input = self.prompt.text(msg)?;

        Ok(input)
    }

    pub fn enter_variant_name(&self) -> Result<String, TemplateError> {
        let msg = "Enter variant name:";
        let input = self.prompt.text(msg)?;

        Ok(input)
    }
}
