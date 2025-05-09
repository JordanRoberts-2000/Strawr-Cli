use crate::{cli::commands::img::ImgCommand, context::AppContext, services::img::Img};

impl ImgCommand {
    pub fn apply_resize(&self, img: &mut Img, ctx: &AppContext) {
        let max_size = match &self.size {
            Some(size) => Some(size.clone().into()),
            None if self.placeholder.unwrap_or(false) => Some(ctx.config.img.placeholder_size),
            None if self.thumbnail.unwrap_or(false) => Some(ctx.config.img.thumbnail_size),
            None => ctx.config.img.max_size,
        };

        let current_width = img.width;
        let current_height = img.height;

        if let Some(size) = max_size {
            if current_width > size || current_height > size {
                img.max_size(size);
                log::info!("Image resized with max size {size}");
            } else {
                log::debug!(
                    "Skipping resize: image already within max size ({}x{}) ≤ {}",
                    current_width,
                    current_height,
                    size
                );
            }
        }
    }
}
