use crate::template::{
    constants::DEFAULT_FOLDER,
    models::{markers::Exists, Template, Variant},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn select_default_or_variant(
        &self,
    ) -> Result<(Template<Exists>, Option<Variant<Exists>>), TemplateError> {
        let template = self.select_template()?;

        if !self.service.has_variants(&template)? {
            return Ok((template, None));
        }

        let variant = self.select_variant_including_default(&template)?;

        if variant.id().to_string() == DEFAULT_FOLDER {
            return Ok((template, None));
        }

        Ok((template, Some(variant)))
    }
}
