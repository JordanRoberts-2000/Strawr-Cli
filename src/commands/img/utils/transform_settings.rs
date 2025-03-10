use crate::{commands::img::ImgCommand, state::AppContext};

use super::valid_image_format::ValidImageFormat;

pub struct TransformSettings {
    pub format: ValidImageFormat,
    // max_size: u32,
    // resize: (u32, u32),
    // crop: (u32, u32),
}

impl ImgCommand {
    pub fn transform_settings(&self, ctx: &AppContext) -> TransformSettings {
        let format = if let Some(ref user_format) = self.format {
            user_format.clone()
        } else {
            ctx.config.img.default_format.clone()
        };

        TransformSettings { format }
    }
}
