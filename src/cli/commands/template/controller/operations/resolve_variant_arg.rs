use crate::template::{
    controller::enums::VariantArgEmpty,
    models::{markers::Exists, Template, Variant},
    types::ValidVariantName,
    TemplateController, TemplateError,
};

impl<'a> TemplateController<'a> {
    pub fn resolve_variant_arg(
        &self,
        variant_arg: &Option<ValidVariantName>,
        template: &Template,
        variant_empty: &VariantArgEmpty,
    ) -> Result<Variant, TemplateError> {
        let variant = match variant_arg {
            None => match variant_empty {
                VariantArgEmpty::Select => self.select_variant(&template)?.into(),
                VariantArgEmpty::PromptText => {
                    let t = template.ensure_exists()?;
                    self.prompt_variant_name(&t)?
                }
            },
            Some(v) => {
                let t: Template<Exists> = template.ensure_exists()?;
                Variant::new(&t, v)
            }
        };

        Ok(variant)
    }
}
