use crate::cli::commands::img::{ImgCommand, ImgError};

impl ImgCommand {
    pub fn handle_directory(&self) -> Result<(), ImgError> {
        Ok(())
    }
}

// Invalid output for file example:
// if !output_path.exists() || !output_path.is_file() {
//   return Err(ImgError::OutputMustBeFile(output.clone()));
// }

// Folder
// for path in handle_folder(&self.input) {
//   let mut img = Img::open(&self.input.join(path))?;

//   self.process_image(img);

//   match &self.output {
//     Some(output) => img.save_to(output.join(path))?,
//     None => img.save()?,
// }
// }
