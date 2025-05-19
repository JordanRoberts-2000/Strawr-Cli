use crate::template::{
    types::{ValidTemplateName, ValidVariantName},
    TemplateError, TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn select_template(
        &self,
        options: &Vec<String>,
    ) -> Result<ValidTemplateName, TemplateError> {
        let msg = "Select template:";
        let input = self.prompt.search(options, msg)?;

        Ok(ValidTemplateName::new(&input))
    }

    pub fn select_variant(&self, options: &Vec<String>) -> Result<ValidVariantName, TemplateError> {
        let msg = "Select variant:";
        let input = self.prompt.search(options, msg)?;

        Ok(ValidVariantName::new(&input))
    }
}
