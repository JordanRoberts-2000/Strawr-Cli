use std::{io, process::Command};

use super::GrabService;

impl GrabService {
    pub fn open_list_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.list_file_path.to_str().unwrap();

        let status = if cfg!(target_os = "macos") {
            log::debug!("Opened using macOS 'TextEdit'");
            Command::new("open")
                .args(&["-a", "TextEdit", path])
                .status()?
        } else if cfg!(target_os = "windows") {
            log::debug!("Opened using Windows 'notepad'");
            Command::new("notepad").arg(path).status()?
        } else if cfg!(target_os = "linux") {
            log::debug!("Opened using Linux 'xdg-open'");
            Command::new("xdg-open").arg(path).status()?
        } else {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported OS",
            )));
        };

        if !status.success() {
            println!("Find keys in '{:?}'", &self.list_file_path);
        }

        Ok(())
    }
}
