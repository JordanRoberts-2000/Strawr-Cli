use crate::template::{
    constants::DEFAULT_FOLDER,
    models::{markers::Exists, Template, Variant},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn select_variant(
        &self,
        template: &Template<Exists>,
    ) -> Result<Variant<Exists>, TemplateError> {
        if !self.service.has_variants(&template)? {
            return Err(TemplateError::NoVariants(template.id().to_string()));
        }

        let mut variants = self.service.get_variants(&template)?;
        variants.retain(|v| v != DEFAULT_FOLDER);
        let input = self.view.select_variant(&variants)?;

        Ok(Variant::new(&template, &input))
    }

    pub fn select_variant_including_default(
        &self,
        template: &Template<Exists>,
    ) -> Result<Variant<Exists>, TemplateError> {
        let variants = self.service.get_variants(&template)?;
        let input = self.view.select_variant(&variants)?;

        Ok(Variant::new(&template, &input))
    }
}
