use std::path::PathBuf;

use crate::{
    cli::commands::img::{ImgConfig, ImgError},
    services::img::Img,
    state::AppContext,
};

pub mod alt;
pub mod blur_url;
pub mod blurhash;
pub mod color;
pub mod data_url;
pub mod default;

pub struct GetManager {
    img: Img,
    config: ImgConfig,
}

impl GetManager {
    pub fn new(path_str: &String, ctx: &AppContext) -> Result<Self, ImgError> {
        let path = PathBuf::from(path_str);
        let img = Img::open(&path)?;
        let config = ctx.config.img.clone();

        Ok(Self { img, config })
    }
}
