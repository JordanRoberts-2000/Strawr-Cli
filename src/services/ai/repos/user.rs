use crate::{
    ai::sync::gen,
    services::ai::{traits::GenImage, AiServiceError},
};

pub struct UserAiRepo {
    api_key: String,
}

impl UserAiRepo {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
        }
    }
}

impl GenImage for UserAiRepo {
    fn gen_image(&self, description: &str) -> Result<String, AiServiceError> {
        let url = gen::image(&self.api_key, description)?.generate()?;
        Ok(url)
    }

    fn get_image_description(&self, url: &str) -> Result<String, AiServiceError> {
        let description = gen::image_description(&self.api_key, url)?;
        Ok(description)
    }
}
