use std::path::PathBuf;

use crate::commands::img::ImgCommand;
use crate::error::{Error, Result};

impl ImgCommand {
    pub fn validate_path(&self) -> Result<PathBuf> {
        let path = self
            .path
            .as_ref()
            .or(self.positional_path.as_ref())
            .ok_or_else(|| {
                log::error!("path should have a value but doesn't");
                Error::Internal
            })?;

        let mut path_buf = std::path::PathBuf::from(path);

        if !path_buf.is_absolute() {
            let current_dir = std::env::current_dir().map_err(|e| {
                log::error!("Could not retrieve current directory, {}", e);
                Error::Internal
            })?;
            path_buf = current_dir.join(path_buf);
        }

        if !path_buf.exists() {
            return Err(Error::Custom(format!(
                "Path '{}' did'nt lead to a valid file or folder",
                path
            )));
        }

        log::trace!("Path validated");
        Ok(path_buf)
    }
}
