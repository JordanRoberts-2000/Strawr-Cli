use std::fs;

use crate::{cli::commands::template::TemplateError, error::IoError};

use super::Template;

impl Template {
    pub fn has_variants(&self) -> Result<bool, TemplateError> {
        let mut entries =
            fs::read_dir(&self.path).map_err(|e| IoError::ReadDir(e, self.path.clone()))?;
        let mut count = 0;

        while let Some(_) = entries.next() {
            count += 1;
            if count >= 2 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
