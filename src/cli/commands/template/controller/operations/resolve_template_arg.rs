use crate::template::{
    models::{Template, Variant},
    types::ParsedTemplateInput,
    TemplateController, TemplateError,
};

impl<'a> TemplateController<'a> {
    pub fn resolve_template_arg(
        &self,
        template_arg: &ParsedTemplateInput,
    ) -> Result<(Template, Option<Variant>), TemplateError> {
        let (template_part, variant_part) = template_arg;
        let template = Template::new(&template_part, &self.service.templates_path);

        if let Some(v) = variant_part {
            let variant = match v {
                None => {
                    self.service.ensure_template_exists(&template)?;
                    self.select_variant(&template)?
                }
                Some(v) => Variant::new(&template, v),
            };
            return Ok((template, Some(variant)));
        }

        Ok((template, None))
    }
}
