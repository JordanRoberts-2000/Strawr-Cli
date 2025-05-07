use std::path::Path;

use crate::{
    cli::commands::template::{TemplateError, TEMPLATES_FOLDER_NAME},
    error::IoError,
};

pub fn templates_folder_init(storage_dir: &Path) -> Result<(), TemplateError> {
    let templates_path = storage_dir.join(TEMPLATES_FOLDER_NAME);

    if !templates_path.exists() {
        std::fs::create_dir_all(&templates_path)
            .map_err(|e| TemplateError::Io(IoError::CreateDir(e, templates_path)))?;
    }

    Ok(())
}
