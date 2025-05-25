use super::{repos::user::UserAiRepo, traits::GenImage, AiServiceError};

use crate::ai::{AiImageModel, ImageSize};

pub trait AiRepo: GenImage {}

impl<T> AiRepo for T where T: GenImage {}

pub struct AiService {
    pub(super) repo: Box<dyn AiRepo>,
}

impl AiService {
    pub fn new(api_key: &str) -> Self {
        Self {
            repo: Box::new(UserAiRepo::new(api_key)),
        }
    }

    pub fn set_repo(&mut self, repo: impl AiRepo + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }

    pub fn generate_image(
        &self,
        description: &str,
        model: AiImageModel,
        size: impl Into<ImageSize>,
    ) -> Result<String, AiServiceError> {
        self.repo.gen_image(description, model, size.into())
    }

    pub fn get_image_description(&self, url: &str) -> Result<String, AiServiceError> {
        self.repo.get_image_description(url)
    }
}
