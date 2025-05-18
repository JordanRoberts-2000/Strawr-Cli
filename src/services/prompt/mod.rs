mod error;
mod service;
pub mod traits;
mod repos {
    pub mod mock;
    pub mod user;
}

pub use {
    error::PromptServiceError,
    repos::{mock, user},
    service::core::PromptService,
};
