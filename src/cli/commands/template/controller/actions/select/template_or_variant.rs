use crate::template::{
    controller::enums::TemplateOrVariant,
    models::{markers::Exists, Template, Variant},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn select_template_or_variant(
        &self,
    ) -> Result<(Template<Exists>, Option<Variant<Exists>>), TemplateError> {
        let template = self.select_template()?;

        if !self.service.has_variants(&template)? {
            return Ok((template, None));
        }

        let choice = self.view.select_template_or_variant()?;
        match choice {
            TemplateOrVariant::Template => Ok((template, None)),
            TemplateOrVariant::Variant => {
                let v = self.select_variant(&template)?;
                Ok((template, Some(v)))
            }
        }
    }
}
