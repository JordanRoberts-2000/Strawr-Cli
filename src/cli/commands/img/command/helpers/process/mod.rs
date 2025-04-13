use std::path::Path;

use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    services::img::Img,
    state::AppContext,
};

pub mod blur;
pub mod conversion;
pub mod output;
pub mod resize;

impl ImgCommand {
    pub fn process_image(
        &self,
        img: &mut Img,
        ctx: &AppContext,
    ) -> Result<Option<String>, ImgError> {
        self.apply_blur(img, ctx);

        self.apply_resize(img, ctx);

        self.apply_conversion(img, ctx)?;

        let output = match &self.output {
            Some(Some(path)) => Some(path.clone()),
            Some(None) => Some(
                img.target_path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .to_string_lossy()
                    .to_string(),
            ),
            None => None,
        };

        Ok(output)
    }
}
