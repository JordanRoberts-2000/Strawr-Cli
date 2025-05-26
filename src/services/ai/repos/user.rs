use crate::ai::blocking::r#gen::PromptBuilder;
use crate::{
    ai::{blocking::gen, AiImageModel, ImageSize},
    services::ai::{
        traits::{GenImage, Prompt},
        AiServiceError,
    },
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
    fn gen_image(
        &self,
        description: &str,
        model: AiImageModel,
        size: ImageSize,
    ) -> Result<String, AiServiceError> {
        let url = gen::image(&self.api_key, description)?
            .model(model)
            .size(size)
            .generate()?;
        Ok(url)
    }

    fn get_image_description(&self, url: &str) -> Result<String, AiServiceError> {
        let description = gen::image_description(&self.api_key, url)?;
        Ok(description)
    }
}

impl Prompt for UserAiRepo {
    fn prompt(&self, prompt: &str, max_tokens: u16) -> Result<String, AiServiceError> {
        let response = gen::prompt(&self.api_key, prompt)?
            .max_tokens(max_tokens)
            .generate()?;
        Ok(response)
    }
}
