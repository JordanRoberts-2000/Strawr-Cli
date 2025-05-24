pub mod error;
pub mod sync;

pub use error::AiError;
pub(crate) use sync::open_ai_client;
