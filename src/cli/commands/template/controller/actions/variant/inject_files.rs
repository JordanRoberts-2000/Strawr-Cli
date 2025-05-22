use std::path::Path;

use crate::{
    template::{
        models::{markers::Exists, Variant},
        TemplateController, TemplateError,
    },
    utils,
};

impl<'c> TemplateController<'c> {
    pub fn inject_variant_files(
        &self,
        variant: &Variant,
        output: &Path,
    ) -> Result<(), TemplateError> {
        let variant: Variant<Exists> = variant.ensure_exists()?;

        if !utils::fs::dir_empty(&output)? && !self.view.output_not_empty_warning()? {
            return Ok(());
        }

        utils::fs::copy_dir_contents(&variant.path(), &output)?;
        Ok(())
    }
}
