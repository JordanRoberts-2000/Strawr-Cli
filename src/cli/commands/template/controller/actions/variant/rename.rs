use crate::template::{
    models::Variant, types::ValidVariantName, TemplateController, TemplateError,
};

impl TemplateController {
    pub fn rename_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        // todo return trusted to go in rename/create ect?
        // todo then remove ensure exists from create/delete/ect
        self.service.ensure_variant_exists(variant)?;

        let input = self.view.enter_variant_name()?;
        let new_name: ValidVariantName = input.parse().map_err(TemplateError::InvalidVariant)?;

        self.service.rename_variant(&variant, &new_name)?;
        self.view.variant_renamed(&variant, &new_name);
        Ok(())
    }
}
