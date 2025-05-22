use crate::template::{
    controller::{enums::VariantArgEmpty, resolver::TemplateResolver},
    types::{ParsedTemplateInput, ValidVariantName},
    TemplateController, TemplateError,
};

impl<'a> TemplateController<'a> {
    pub fn resolve_template(
        &self,
        template_arg: &ParsedTemplateInput,
        variant_arg: &Option<Option<ValidVariantName>>,
        variant_arg_empty: VariantArgEmpty,
    ) -> Result<TemplateResolver, TemplateError> {
        let (template, variant_suffix) =
            self.resolve_template_arg(template_arg, &variant_arg_empty)?;

        if let Some(v) = variant_suffix {
            if variant_arg.is_some() {
                self.view.warn_variant_ignored();
            }
            return Ok(TemplateResolver::new(self, template, Some(v)));
        }

        if let Some(v) = variant_arg {
            let resolved_variant = self.resolve_variant_arg(v, &template, &variant_arg_empty)?;
            return Ok(TemplateResolver::new(
                self,
                template,
                Some(resolved_variant),
            ));
        }

        Ok(TemplateResolver::new(self, template, None))
    }
}
