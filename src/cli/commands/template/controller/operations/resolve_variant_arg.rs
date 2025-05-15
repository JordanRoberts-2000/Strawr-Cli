use crate::template::{
    models::{Template, Variant},
    types::ValidVariantName,
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn resolve_variant_arg(
        &self,
        variant_arg: &Option<ValidVariantName>,
        template: &Template,
    ) -> Result<Variant, TemplateError> {
        let variant = match variant_arg {
            None => self.select_variant(template)?,
            Some(v) => Variant::new(template, v),
        };

        Ok(variant)
    }
}
