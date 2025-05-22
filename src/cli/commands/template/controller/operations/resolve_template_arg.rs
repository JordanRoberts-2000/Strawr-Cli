use crate::template::{
    controller::enums::VariantArgEmpty,
    models::{Template, Variant},
    types::ParsedTemplateInput,
    TemplateController, TemplateError,
};

impl<'a> TemplateController<'a> {
    pub fn resolve_template_arg(
        &self,
        template_arg: &ParsedTemplateInput,
        variant_arg_empty: &VariantArgEmpty,
    ) -> Result<(Template, Option<Variant>), TemplateError> {
        let (template_part, variant_part) = template_arg;
        let template = Template::new(&template_part, &self.service.templates_path);

        if let Some(v) = variant_part {
            let variant = self.resolve_variant_arg(v, &template, variant_arg_empty)?;
            return Ok((template, Some(variant)));
        }

        Ok((template, None))
    }
}
