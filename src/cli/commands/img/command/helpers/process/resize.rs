use crate::{cli::commands::img::ImgCommand, services::img::Img, state::AppContext};

impl ImgCommand {
    pub fn apply_resize(&self, img: &mut Img, ctx: &AppContext) {
        let max_size = match &self.size {
            Some(size) => Some(size.clone().into()),
            None if self.placeholder.unwrap_or(false) => Some(ctx.config.img.placeholder_size),
            None if self.thumbnail.unwrap_or(false) => Some(ctx.config.img.thumbnail_size),
            None => ctx.config.img.max_size,
        };

        if let Some(size) = max_size {
            img.resize(size, size);
            log::info!("Image given max size {size}");
        }
    }
}
