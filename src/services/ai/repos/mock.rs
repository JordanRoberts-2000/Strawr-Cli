use crate::services::ai::{traits::GenImage, AiServiceError};
use std::{cell::RefCell, collections::VecDeque};

#[derive(Debug)]
pub enum MockAiOutput {
    ImageUrl(String),
    ImageDescription(String),
}

pub struct MockAiRepo {
    responses: RefCell<VecDeque<MockAiOutput>>,
}

impl MockAiRepo {
    pub fn new(responses: Vec<MockAiOutput>) -> Self {
        MockAiRepo {
            responses: RefCell::new(VecDeque::from(responses)),
        }
    }
}

impl GenImage for MockAiRepo {
    fn gen_image(&self, _description: &str) -> Result<String, AiServiceError> {
        let mut q = self.responses.borrow_mut();
        match q.pop_front() {
            Some(MockAiOutput::ImageUrl(url)) => Ok(url),
            Some(other) => panic!(
                "MockAiRepo: expected ImageUrl, got {:?} in gen_image()",
                other
            ),
            None => panic!("MockAiRepo: no more responses left in gen_image()"),
        }
    }

    fn get_image_description(&self, _url: &str) -> Result<String, AiServiceError> {
        let mut q = self.responses.borrow_mut();
        match q.pop_front() {
            Some(MockAiOutput::ImageDescription(desc)) => Ok(desc),
            Some(other) => panic!(
                "MockAiRepo: expected ImageDescription, got {:?} in get_image_description()",
                other
            ),
            None => panic!("MockAiRepo: no more responses left in get_image_description()"),
        }
    }
}
