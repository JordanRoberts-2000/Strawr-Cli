use builder::ImageBuilder;

pub mod builder;
pub mod models;

pub fn image<A, D>(api_key: A, description: D) -> ImageBuilder
where
    A: Into<String>,
    D: Into<String>,
{
    ImageBuilder::new(api_key, description)
}
