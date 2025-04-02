use std::process::Command;

use crate::cli::commands::grab::GrabError;

use super::GrabManager;

impl GrabManager {
    pub fn open_list_file(&self) -> Result<(), GrabError> {
        log::trace!("Attempting to open list file");

        let path = self.json_file_path.display().to_string();

        let status = if cfg!(target_os = "macos") {
            log::debug!("Opened using macOS 'TextEdit'");
            Command::new("open")
                .args(&["-a", "TextEdit", &path])
                .status()
        } else if cfg!(target_os = "windows") {
            log::debug!("Opened using Windows 'notepad'");
            Command::new("notepad").arg(&path).status()
        } else if cfg!(target_os = "linux") {
            log::debug!("Opened using Linux 'xdg-open'");
            Command::new("xdg-open").arg(&path).status()
        } else {
            log::warn!("Unsupported OS");
            Command::new("false").status()
        }
        .map_err(|e| GrabError::Io {
            source: e,
            context: format!("Failed to open list file '{}'", &path),
        })?;

        if !status.success() {
            println!("Find keys in '{:?}'", &self.list_file_path);
        } else {
            println!("Opened list file")
        }

        Ok(())
    }
}
