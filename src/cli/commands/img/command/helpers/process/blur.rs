use crate::{cli::commands::img::ImgCommand, services::img::Img, state::AppContext};

impl ImgCommand {
    pub fn apply_blur(&self, img: &mut Img, ctx: &AppContext) {
        if let Some(blur) = &self.blur {
            img.blur(blur.unwrap_or(ctx.config.img.blur_intensity));
        } else if self.placeholder.unwrap_or(false) {
            img.blur(ctx.config.img.placeholder_blur_intensity);
        }
    }
}
