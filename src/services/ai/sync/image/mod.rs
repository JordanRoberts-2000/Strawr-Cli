use builder::ImageBuilder;

pub mod builder;
pub mod models;

pub fn image<S: Into<String>>(api_key: S, description: S) -> ImageBuilder {
    ImageBuilder::new(api_key, description)
}
