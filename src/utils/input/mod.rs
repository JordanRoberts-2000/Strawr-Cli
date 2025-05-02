mod confirm;
mod error;
mod select;
mod text;

use std::{cell::RefCell, collections::VecDeque};

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

pub trait CliInput: ConfirmInput + TextInput + SelectInput {}
impl<T: ConfirmInput + TextInput + SelectInput> CliInput for T {}

pub struct UserInput;

pub struct TestInput {
    pub inputs: RefCell<VecDeque<Input>>,
}

impl TestInput {
    pub fn new() -> Self {
        Self {
            inputs: RefCell::new(VecDeque::new()),
        }
    }

    pub fn from(inputs: Vec<Input>) -> Self {
        Self {
            inputs: RefCell::new(VecDeque::from(inputs)),
        }
    }
}
