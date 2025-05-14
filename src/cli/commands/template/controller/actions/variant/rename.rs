use crate::template::{
    models::Variant, types::ValidVariantName, TemplateController, TemplateError,
};

impl TemplateController {
    pub fn rename_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        let input = self.view.enter_variant_name()?;
        let new_name: ValidVariantName = input.parse().map_err(TemplateError::InvalidVariant)?;

        self.service.rename_variant(&variant, &new_name)?;
        self.view.variant_renamed(&variant, &new_name);
        Ok(())
    }
}
