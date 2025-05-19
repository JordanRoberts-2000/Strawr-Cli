use std::{cell::RefCell, collections::VecDeque};

use crate::services::prompt::mock::{MockInput, MockInputCall};

pub struct MockInputRepo {
    pub inputs: RefCell<VecDeque<MockInput>>,
    pub history: RefCell<Vec<MockInputCall>>,
}

impl MockInputRepo {
    pub fn new() -> Self {
        Self {
            inputs: RefCell::new(VecDeque::new()),
            history: RefCell::new(Vec::new()),
        }
    }

    pub fn from(inputs: Vec<MockInput>) -> Self {
        Self {
            inputs: RefCell::new(VecDeque::from(inputs)),
            history: RefCell::new(Vec::new()),
        }
    }

    pub fn call_count(&self) -> usize {
        self.history.borrow().len()
    }

    pub fn print_history(&self) {
        println!("{:#?}", self.history);
    }

    pub fn last(&self) -> Option<MockInputCall> {
        self.history.borrow().last().cloned()
    }
}
