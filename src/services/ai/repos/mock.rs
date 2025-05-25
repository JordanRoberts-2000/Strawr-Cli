use crate::{
    ai::{AiImageModel, ImageSize},
    services::ai::{traits::GenImage, AiServiceError},
};
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
    fn gen_image(
        &self,
        _description: &str,
        _model: AiImageModel,
        _size: ImageSize,
    ) -> Result<String, AiServiceError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::{AiImageModel, ImageSize};

    #[test]
    fn gen_image_and_description_in_order() {
        let mock = MockAiRepo::new(vec![
            MockAiOutput::ImageUrl("http://example.com/foo.png".into()),
            MockAiOutput::ImageDescription("A foo image".into()),
        ]);

        // 1st call => ImageUrl variant
        let url = mock
            .gen_image("anything", AiImageModel::Dalle2, ImageSize::Lg)
            .unwrap();
        assert_eq!(url, "http://example.com/foo.png");

        // 2nd call => ImageDescription variant
        let desc = mock.get_image_description(&url).unwrap();
        assert_eq!(desc, "A foo image");
    }

    #[test]
    #[should_panic(expected = "expected ImageUrl")]
    fn panic_if_wrong_variant_for_gen_image() {
        // We only queue a description, so gen_image() will see the wrong variant.
        let mock = MockAiRepo::new(vec![MockAiOutput::ImageDescription("oops".into())]);
        // This should panic with “expected ImageUrl…”
        let _ = mock.gen_image("anything", AiImageModel::Dalle2, ImageSize::Lg);
    }

    #[test]
    #[should_panic(expected = "no more responses")]
    fn panic_if_no_responses_for_gen_image() {
        let mock = MockAiRepo::new(vec![]);
        let _ = mock.gen_image("anything", AiImageModel::Dalle2, ImageSize::Lg);
    }

    #[test]
    #[should_panic(expected = "expected ImageDescription")]
    fn panic_if_wrong_variant_for_get_image_description() {
        // We only queue a URL, so get_image_description() will see the wrong variant.
        let mock = MockAiRepo::new(vec![MockAiOutput::ImageUrl(
            "http://example.com/foo.png".into(),
        )]);
        let _ = mock.get_image_description("http://example.com/foo.png");
    }

    #[test]
    #[should_panic(expected = "no more responses")]
    fn panic_if_no_responses_for_get_image_description() {
        let mock = MockAiRepo::new(vec![]);
        let _ = mock.get_image_description("http://example.com/foo.png");
    }
}
