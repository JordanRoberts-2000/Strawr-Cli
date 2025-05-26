mod error;
mod image;
pub mod client {
    pub mod blocking;
}
mod prompt {
    pub mod blocking;
}
pub mod models;

pub(super) use error::AiResult;

pub use {error::AiError, image::enums::*};

pub mod blocking {
    pub mod gen {
        pub use super::super::image::blocking::*;
        pub use super::super::prompt::blocking::*;
    }
}
