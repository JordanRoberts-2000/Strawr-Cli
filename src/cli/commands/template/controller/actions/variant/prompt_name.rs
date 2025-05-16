use crate::template::{
    models::{Template, Variant},
    types::ValidVariantName,
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn prompt_variant_name(&self, template: &Template) -> Result<Variant, TemplateError> {
        let input = self.view.enter_variant_name()?;
        let new_name: ValidVariantName = input.parse().map_err(TemplateError::InvalidVariant)?;

        Ok(Variant::new(template, &new_name))
    }
}
