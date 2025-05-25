mod error;
mod image;

pub(super) use error::AiResult;

pub use {error::AiError, image::enums::*};

pub mod blocking {
    pub use super::image::blocking::gen;
}
