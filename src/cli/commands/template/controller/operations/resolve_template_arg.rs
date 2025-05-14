use crate::template::{
    models::{Template, Variant},
    types::ParsedTemplateInput,
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn resolve_template_arg(
        &self,
        template_arg: &ParsedTemplateInput,
    ) -> Result<(Template, Option<Variant>), TemplateError> {
        let (template_part, variant_part) = template_arg;
        let template = self.service.get_existing_template(template_part)?;

        if let Some(v) = variant_part {
            let variant = match v {
                None => self.select_variant(&template)?,
                Some(v) => self.service.get_existing_variant(&template, v)?,
            };
            return Ok((template, Some(variant)));
        }

        Ok((template, None))
    }
}
