use std::{fs, path::Path};

pub fn trash(path: &Path) -> Result<(), std::io::Error> {
    if let Err(trash_err) = trash::delete(path) {
        log::warn!(
            "Failed to delete file {:?} via trash::delete: {:?}. Falling back to fs::remove_file.",
            path,
            trash_err
        );
        fs::remove_file(path)?;
    }

    Ok(())
}
