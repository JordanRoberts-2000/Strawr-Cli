mod core;
mod error;
pub mod adaptors {
    pub mod clap;
}

pub use error::ValidationError;
pub mod validate {
    pub use super::core::*;
}
