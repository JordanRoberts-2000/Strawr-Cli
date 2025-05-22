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
        let t = template.ensure_exists()?;

        let variant = match variant_arg {
            None => match variant_empty {
                VariantArgEmpty::Select => self.select_variant(&t)?.into(),
                VariantArgEmpty::PromptText => self.prompt_variant_name(&t)?,
            },
            Some(v) => {
                let t: Template<Exists> = template.ensure_exists()?;
                Variant::new(&t, v)
            }
        };

        Ok(variant)
    }
}
