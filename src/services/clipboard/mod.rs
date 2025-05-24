mod error;
mod service;
mod traits;
mod repos {
    pub mod mock;
    pub mod user;
}

pub use {
    error::ClipboardError,
    repos::{mock::MockClipboardRepo, user::UserClipboardRepo},
    service::ClipboardService,
    traits::*,
};
