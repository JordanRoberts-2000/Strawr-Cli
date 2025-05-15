use std::path::{Path, PathBuf};

use crate::template::{constants::DEFAULT_FOLDER, types::ValidTemplateName};

#[derive(Debug, Clone)]
pub struct Template {
    pub id: ValidTemplateName,
    pub path: PathBuf,
}

impl Template {
    pub fn new(id: &ValidTemplateName, templates_path: &Path) -> Self {
        let path = templates_path.join(id.as_str());
        Self {
            id: id.clone(),
            path,
        }
    }

    pub fn default_path(&self) -> PathBuf {
        self.path.join(DEFAULT_FOLDER)
    }
}
