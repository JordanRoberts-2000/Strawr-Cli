use crate::template::{
    controller::resolver::TemplateResolver,
    types::{ParsedTemplateInput, ValidVariantName},
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn resolve_template<'a>(
        &'a self,
        template_arg: &ParsedTemplateInput,
        variant_arg: &Option<Option<ValidVariantName>>,
    ) -> Result<TemplateResolver<'a>, TemplateError> {
        let (template, variant) = self.resolve_template_arg(&template_arg)?;

        if let Some(v) = variant {
            if variant_arg.is_some() {
                self.view.warn_variant_ignored();
            }
            return Ok(TemplateResolver::new(self, template, Some(v)));
        }

        if let Some(v) = variant_arg {
            let resolved_variant = self.resolve_variant_arg(v, &template)?;
            return Ok(TemplateResolver::new(
                self,
                template,
                Some(resolved_variant),
            ));
        }

        Ok(TemplateResolver::new(self, template, None))
    }
}
