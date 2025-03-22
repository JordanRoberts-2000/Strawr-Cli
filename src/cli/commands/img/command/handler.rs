use std::path::PathBuf;

use crate::error::Result;
use crate::services::img::Img;
use crate::state::AppContext;

use super::args::ImgCommand;

impl ImgCommand {
    pub fn execute(&self, _ctx: &AppContext) -> Result<()> {
        let temp_input_path = PathBuf::from(
            "/Users/jordanroberts/Documents/dev/Projects/main/rustCli/playground/img/croc.webp",
        );
        let temp_output_path = PathBuf::from(
            "/Users/jordanroberts/Documents/dev/Projects/main/rustCli/playground/img/croc.jpg",
        );

        let mut img = Img::new(&temp_input_path).expect("Failed to open image");
        img.max_size(600)
            .jpeg(100)
            .expect("failed to convert to web p")
            .save()
            .expect("failed to save");

        Ok(())
    }
}
