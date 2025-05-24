mod image_decription;
mod img_gen;
mod models {
    pub mod image_description;
    pub mod image_gen;
}

pub(crate) use models::{image_description::ImageDescriptionResponse, image_gen::ImageGenResponse};
pub use {image_decription::image_description, img_gen::*};
