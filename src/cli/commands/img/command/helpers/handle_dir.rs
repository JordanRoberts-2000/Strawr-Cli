use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    context::AppContext,
    services::img::Img,
};

use super::supported::is_supported_image;

impl ImgCommand {
    pub fn handle_directory(&self, input: &String, ctx: &AppContext) -> Result<(), ImgError> {
        let input_path = Path::new(input);

        for entry in WalkDir::new(input).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if path.is_file() && is_supported_image(path) {
                log::debug!("Processing: {:?}", path);
                let relative_path = path.strip_prefix(input_path).unwrap();

                let mut img = Img::open(path)?;

                match self.process_image(&mut img, ctx)? {
                    Some(output_dir) => {
                        let base_path = PathBuf::from(output_dir);
                        let output =
                            base_path.join(relative_path.parent().unwrap_or_else(|| Path::new("")));

                        img.save_to(output)?
                    }
                    None => img.save()?,
                }
            }
        }

        Ok(())
    }
}

// for path in handle_folder(&self.input) {
//   let mut img = Img::open(&self.input.join(path))?;

//   self.process_image(img);

//   match &self.output {
//     Some(output) => img.save_to(output.join(path))?,
//     None => img.save()?,
// }
// }
