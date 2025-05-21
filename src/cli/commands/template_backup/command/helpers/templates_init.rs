use std::path::Path;

use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateError},
    utils,
};

pub fn templates_folder_init(storage_dir: &Path) -> Result<(), TemplateError> {
    let templates_path = storage_dir.join(TEMPLATES_FOLDER_NAME);

    if !templates_path.exists() {
        utils::fs::create_dir_all(&templates_path)?;
    }

    Ok(())
}
