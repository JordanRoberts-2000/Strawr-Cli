use crate::{cli::commands::img::ImgCommand, context::AppContext, services::img::Img};

impl ImgCommand {
    pub fn apply_blur(&self, img: &mut Img, ctx: &AppContext) {
        let blur_intensity = if let Some(blur) = &self.blur {
            let intensity = blur.unwrap_or(ctx.config.img.blur_intensity);
            Some(intensity)
        } else if self.placeholder.unwrap_or(false) {
            Some(ctx.config.img.placeholder_blur_intensity)
        } else {
            None
        };

        if let Some(intensity) = blur_intensity {
            img.blur(intensity);
            log::info!("Blur applied to image with intensity: {}", intensity);
        }
    }
}
