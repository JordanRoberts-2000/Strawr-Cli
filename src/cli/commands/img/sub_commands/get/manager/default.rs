use crate::cli::commands::img::ImgError;

use super::GetManager;

fn format_size(size_bytes: usize) -> String {
    if size_bytes < 1024 {
        format!("{}Bytes", size_bytes)
    } else if size_bytes < 1024 * 1024 {
        format!("{:.1}Kb", size_bytes as f64 / 1024.0)
    } else {
        format!("{:.1}Mb", size_bytes as f64 / (1024.0 * 1024.0))
    }
}

impl GetManager {
    pub fn handle_default(&mut self) -> Result<(), ImgError> {
        println!(
            "{:?} - {}x{} - {}",
            self.img.format,
            self.img.width,
            self.img.height,
            format_size(self.img.size_bytes)
        );
        Ok(())
    }
}
