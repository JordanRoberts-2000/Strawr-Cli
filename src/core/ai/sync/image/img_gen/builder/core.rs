use crate::{
    ai::{
        sync::r#gen::{AiImageModel, ImageSize},
        AiError,
    },
    utils::validation::validate,
};

pub struct ImageGenBuilder {
    pub(super) api_key: String,
    pub(super) description: String,
    pub(super) model: AiImageModel,
    pub(super) size: ImageSize,
}

impl ImageGenBuilder {
    pub fn new(
        api_key: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, AiError> {
        let api_key = validate::is_empty(api_key.into())?;
        let description = validate::is_empty(description.into())?;

        Ok(Self {
            api_key,
            description,
            model: AiImageModel::default(),
            size: ImageSize::default(),
        })
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    pub fn model(mut self, model: AiImageModel) -> Self {
        self.model = model;
        self
    }
}
