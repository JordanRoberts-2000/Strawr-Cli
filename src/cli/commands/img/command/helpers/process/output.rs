use std::path::PathBuf;

use crate::cli::commands::img::ImgCommand;

impl ImgCommand {
    pub fn handle_output(&self) -> PathBuf {
        // if let Some(output) = &self.output {
        //   return output.unwrap_or()
        // } else {

        // }

        PathBuf::new()
    }
}
