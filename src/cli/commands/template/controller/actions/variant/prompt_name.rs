use crate::template::{types::ValidVariantName, TemplateController, TemplateError};

impl TemplateController {
    pub fn prompt_variant_name(&self) -> Result<ValidVariantName, TemplateError> {
        let input = self.view.enter_variant_name()?;
        let template: ValidVariantName = input.parse().map_err(TemplateError::InvalidVariant)?;
        Ok(template)
    }
}
