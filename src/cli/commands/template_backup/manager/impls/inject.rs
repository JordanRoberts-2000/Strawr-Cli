use std::path::Path;

use crate::{
    error::IoError,
    template::{
        constants::DEFAULT_FOLDER,
        utils::{Template, Variant},
        TemplateError, TemplateManager,
    },
    utils::fs::{copy_dir_contents, is_dir_empty},
};

impl<'a> TemplateManager<'a> {
    pub fn inject(
        &self,
        template: &Template,
        variant: Option<&Variant>,
        output: &Path,
    ) -> Result<(), TemplateError> {
        let source_path = template.path.join(
            variant
                .map(|v| v.path.as_path())
                .unwrap_or(DEFAULT_FOLDER.as_ref()),
        );

        let output_empty =
            is_dir_empty(output).map_err(|e| IoError::ReadDir(e, output.to_path_buf()))?;

        if !output_empty {
            let msg =
                "The output directory is not empty. Do you still want to inject template files?";
            if !self.service.prompt().confirm(msg)? {
                return Ok(());
            }
        }

        copy_dir_contents(&source_path, output)
            .map_err(|e| IoError::CopyDirContents(e, source_path.clone(), output.to_path_buf()))?;

        Ok(())
    }
}
