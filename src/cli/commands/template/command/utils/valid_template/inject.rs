use std::path::PathBuf;

use crate::{
    cli::commands::template::{TemplateError, DEFAULT_FOLDER},
    error::IoError,
    utils::fs::copy_dir_contents,
};

use super::Template;

impl Template {
    pub fn inject(&self, output: &PathBuf, variant: Option<&str>) -> Result<(), TemplateError> {
        let source_path = &self.path.join(variant.unwrap_or(DEFAULT_FOLDER));

        copy_dir_contents(&source_path, &output)
            .map_err(|e| IoError::CopyDirContents(e, source_path.clone(), output.clone()))?;

        Ok(())
    }
}
