use std::path::Path;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::Img,
};

impl ImgCommand {
    pub fn apply_file_name(&self, img: &mut Img) -> Result<(), ImgError> {
        let original_file_name = Path::new(&img.file_name()?)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut new_file_name = self.rename.clone().unwrap_or(original_file_name.clone());

        if let Some(prefix) = &self.prefix {
            new_file_name = format!("{}{}", prefix, new_file_name);
        }

        if let Some(suffix) = &self.suffix {
            new_file_name = format!("{}{}", new_file_name, suffix);
        }

        if original_file_name != new_file_name {
            img.with_file_name(&new_file_name)?;
            log::info!("New filename '{new_file_name}' applied to image");
        }

        Ok(())
    }
}
