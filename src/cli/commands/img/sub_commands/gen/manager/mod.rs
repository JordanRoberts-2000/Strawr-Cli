use crate::{
    services::ai::sync::{ImageSize, Model},
    state::AppContext,
};

use super::args::Gen;

pub mod args;

pub struct GenManager {
    args: Gen,
    pub model: Model,
    pub size: ImageSize,
}

impl GenManager {
    pub fn new(ctx: &AppContext, args: &Gen) -> Self {
        Self {
            args: args.clone(),
            model: ctx.config.img.gen.default_ai_model.clone(),
            size: ctx.config.img.gen.default_img_size.clone(),
        }
    }
}
