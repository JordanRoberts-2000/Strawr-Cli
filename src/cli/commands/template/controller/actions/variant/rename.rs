use crate::template::{
    models::{
        markers::{DoesNotExist, Exists},
        Variant,
    },
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn rename_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        let variant: Variant<Exists> = variant.ensure_exists()?;

        let new_variant: Variant<DoesNotExist> = self
            .prompt_variant_name(&variant.template())?
            .ensure_does_not_exist()?;

        self.service.rename_variant(&variant, &new_variant)?;
        self.view.variant_renamed(&variant, &new_variant);
        Ok(())
    }
}
