pub mod error;
pub mod service;
pub mod traits;
pub mod repos {
    pub mod mock;
    pub mod user;
}

pub use error::AiServiceError;
