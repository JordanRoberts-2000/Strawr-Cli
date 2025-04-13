use crate::{cli::commands::img::ImgCommand, services::img::Img, state::AppContext};

impl ImgCommand {
    pub fn apply_resize(&self, img: &mut Img, ctx: &AppContext) {
        let resize_dimensions = if let Some(size) = &self.size {
            let resize_to: u32 = size.into();
            Some((resize_to, resize_to))
        } else if self.placeholder.unwrap_or(false) {
            let size = ctx.config.img.placeholder_size;
            Some((size, size))
        } else if self.thumbnail.unwrap_or(false) {
            let size = ctx.config.img.thumbnail_size;
            Some((size, size))
        } else if let Some(size) = self.max.or(ctx.config.img.max_size) {
            Some((size, size))
        } else {
            None
        };

        if let Some((width, height)) = resize_dimensions {
            img.resize(width, height);
            log::info!("Image resized to {}x{}", width, height);
        }
    }
}
