use crate::{
    cli::commands::img::{utils::enums::ImageSize, ImgConfig},
    state::AppContext,
};

use super::args::Gen;

pub mod args;

pub struct GenManager {
    args: Gen,
    pub dalle_version: u8,
    pub size: &'static str,
}

impl GenManager {
    pub fn new(ctx: &AppContext, args: &Gen) -> Self {
        Self {
            args: args.clone(),
            dalle_version: ctx.config.img.default_dalle_version,
            size: ctx.config.img.default_dalle_size.resolution(),
        }
    }
}
