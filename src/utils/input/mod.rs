mod confirm;
mod error;
mod select;
mod text;

pub use confirm::ConfirmInput;
pub use error::InputError;
pub use select::{standard::SelectInput, without_filter::SelectWithoutFilterInput};
pub use text::TextInput;

pub struct UserInput;
