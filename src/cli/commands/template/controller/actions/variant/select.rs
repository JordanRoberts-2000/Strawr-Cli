use crate::template::{
    constants::DEFAULT_FOLDER,
    models::{Template, Variant},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn select_variant(&self, template: &Template) -> Result<Variant, TemplateError> {
        self.service.ensure_template_exists(template)?;

        let mut variants = self.service.get_variants(&template)?;
        variants.retain(|v| v != DEFAULT_FOLDER);
        let input = self.view.select_variant(&variants)?;

        Ok(Variant::new(template, &input))
    }

    pub fn select_variant_including_default(
        &self,
        template: &Template,
    ) -> Result<Variant, TemplateError> {
        self.service.ensure_template_exists(template)?;

        let variants = self.service.get_variants(&template)?;
        let input = self.view.select_variant(&variants)?;

        Ok(Variant::new(template, &input))
    }
}
