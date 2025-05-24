use super::AiServiceError;

pub trait GenImage {
    fn gen_image(&self, description: &str) -> Result<String, AiServiceError>;

    fn get_image_description(&self, url: &str) -> Result<String, AiServiceError>;
}
