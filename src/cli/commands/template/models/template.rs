use std::path::{Path, PathBuf};

use crate::template::{constants::DEFAULT_FOLDER, types::ValidTemplateName};

pub struct Template {
    pub name: ValidTemplateName,
    pub path: PathBuf,
}

impl Template {
    pub fn new(name: &ValidTemplateName, templates_path: &Path) -> Self {
        let path = templates_path.join(name.as_str());
        Self {
            name: name.clone(),
            path,
        }
    }

    pub fn default_path(&self) -> PathBuf {
        self.path.join(DEFAULT_FOLDER)
    }
}
