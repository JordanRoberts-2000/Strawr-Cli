use crate::{cli::commands::img::ImgCommand, services::img::Img, state::AppContext};

impl ImgCommand {
    pub fn apply_resize(&self, img: &mut Img, ctx: &AppContext) {
        if let Some(size) = &self.size {
            let resize_to: u32 = size.into();
            img.resize(resize_to, resize_to);
        } else if self.placeholder.unwrap_or(false) {
            img.resize(
                ctx.config.img.placeholder_size,
                ctx.config.img.placeholder_size,
            );
        } else if self.thumbnail.unwrap_or(false) {
            img.resize(ctx.config.img.thumbnail_size, ctx.config.img.thumbnail_size);
        } else if let Some(size) = self.max.or(ctx.config.img.max_size) {
            img.resize(size, size);
        }
    }
}
