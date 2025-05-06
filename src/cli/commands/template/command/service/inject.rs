use std::path::Path;

use crate::{
    cli::commands::template::{
        command::utils::{Template, Variant},
        TemplateError, DEFAULT_FOLDER,
    },
    error::IoError,
    utils::fs::{copy_dir_contents, is_dir_empty},
};

use super::TemplateService;

impl<'a> TemplateService<'a> {
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
            if !self.confirm(msg)? {
                return Ok(());
            }
        }

        copy_dir_contents(&source_path, output)
            .map_err(|e| IoError::CopyDirContents(e, source_path.clone(), output.to_path_buf()))?;

        Ok(())
    }
}
