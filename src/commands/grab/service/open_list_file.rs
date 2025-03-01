use crate::error::{Error, Result};
use std::process::Command;

use super::GrabService;

impl GrabService {
    pub fn open_list_file(&self) -> Result<()> {
        let path = self.list_file_path.to_str().unwrap();

        let status = if cfg!(target_os = "macos") {
            log::debug!("Opened using macOS 'TextEdit'");
            Command::new("open")
                .args(&["-a", "TextEdit", path])
                .status()
        } else if cfg!(target_os = "windows") {
            log::debug!("Opened using Windows 'notepad'");
            Command::new("notepad").arg(path).status()
        } else if cfg!(target_os = "linux") {
            log::debug!("Opened using Linux 'xdg-open'");
            Command::new("xdg-open").arg(path).status()
        } else {
            log::debug!("Unsupported OS, using status with error code");
            Command::new("false").status()
        }
        .map_err(|e| Error::Custom(format!("Failed to open list file, {}", e)))?;

        if !status.success() {
            println!("Find keys in '{:?}'", &self.list_file_path);
        }

        Ok(())
    }
}
