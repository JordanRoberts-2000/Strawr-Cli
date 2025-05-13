use std::path::{Path, PathBuf};

use crate::{
    template::{constants::DEFAULT_FOLDER, TemplateError, TemplateSubcommand},
    utils::validation::{reserved, slug},
};

pub struct Template {
    pub name: String,
    pub path: PathBuf,
}

impl Template {
    pub fn new(name: &str, templates_path: &Path) -> Self {
        let path = templates_path.join(&name);
        Self {
            name: name.to_string(),
            path,
        }
    }

    pub fn validate_name(name: &str) -> Result<String, TemplateError> {
        let template = slug(name).map_err(TemplateError::Validation)?;
        reserved::<TemplateSubcommand>(&template).map_err(TemplateError::Validation)?;

        Ok(template)
    }

    pub fn default_path(&self) -> PathBuf {
        self.path.join(DEFAULT_FOLDER)
    }
}
