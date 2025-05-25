use crate::{
    ai::{
        image::enums::{AiImageModel, ImageSize},
        AiError,
    },
    validation::validate,
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
        let api_key = validate::not_empty(api_key.into())?;
        let description = validate::not_empty(description.into())?;

        Ok(Self {
            api_key,
            description,
            model: AiImageModel::default(),
            size: ImageSize::default(),
        })
    }

    pub fn size(mut self, size: impl Into<ImageSize>) -> Self {
        self.size = size.into();
        self
    }

    pub fn model(mut self, model: AiImageModel) -> Self {
        self.model = model;
        self
    }
}
