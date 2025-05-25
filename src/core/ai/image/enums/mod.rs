mod ai_model;
mod sizes {
    pub mod all;
    pub mod dalle_2;
    pub mod dalle_3;
}

pub use {
    ai_model::AiImageModel,
    sizes::{all::ImageSize, dalle_2::Dalle2ImageSize, dalle_3::Dalle3ImageSize},
};
