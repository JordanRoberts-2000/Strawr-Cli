use std::path::Path;

use crate::{
    template::{
        models::{markers::Exists, Template},
        TemplateController, TemplateError,
    },
    utils,
};

impl<'c> TemplateController<'c> {
    pub fn inject_template_files(
        &self,
        template: &Template,
        output: &Path,
    ) -> Result<(), TemplateError> {
        let template: Template<Exists> = template.ensure_exists()?;

        if !utils::fs::dir_empty(&output)? && !self.view.output_not_empty_warning()? {
            return Ok(());
        }

        utils::fs::copy_dir_contents(&template.default_path(), &output)?;
        Ok(())
    }
}
