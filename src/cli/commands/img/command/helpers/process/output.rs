use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::Img,
};

impl ImgCommand {
    pub fn handle_output(&self, img: &mut Img) -> Result<Option<String>, ImgError> {
        let output = match &self.output {
            Some(Some(path)) => Some(path.clone()),
            Some(None) => Some(
                img.target_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .to_string_lossy()
                    .to_string(),
            ),
            None => None,
        };

        Ok(output)
    }
}
