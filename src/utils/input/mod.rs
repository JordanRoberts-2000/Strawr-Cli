mod confirm;
mod error;
mod select;
mod text;

use std::cell::RefCell;

pub use confirm::ConfirmInput;
pub use error::InputError;
pub use select::{standard::SelectInput, without_filter::SelectWithoutFilterInput};
pub use text::TextInput;

pub enum Input {
    Confirm(bool),
    Text(String),
    Select(String),
    SelectWithoutFilter(String),
}

pub struct UserInput;

pub struct TestInput {
    pub inputs: RefCell<Vec<Input>>,
}

impl TestInput {
    pub fn new(inputs: Vec<Input>) -> Self {
        Self {
            inputs: RefCell::new(inputs),
        }
    }
}
