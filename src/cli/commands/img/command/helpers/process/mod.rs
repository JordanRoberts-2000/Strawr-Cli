use std::path::PathBuf;

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
    ) -> Result<Option<PathBuf>, ImgError> {
        self.apply_blur(img, ctx);

        self.apply_resize(img, ctx);

        self.apply_conversion(img, ctx)?;

        // let output = if let Some(output) = &self.output {
        //     match output {
        //         Some(path) => Some(path.clone()),
        //         None => {
        //             let cwd = std::env::current_dir().map_err(|e| ImgError::)?;
        //             Some(cwd)
        //         }
        //     }
        // } else {
        //     None
        // };

        Ok(None)
    }
}
