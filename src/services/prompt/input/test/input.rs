use std::{cell::RefCell, collections::VecDeque};

use crate::services::prompt::types::Input;

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
