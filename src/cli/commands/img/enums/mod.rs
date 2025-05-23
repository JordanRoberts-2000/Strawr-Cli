mod image_format;
mod image_input;
mod image_size;

pub use {image_format::ValidImageFormat, image_input::ImageInput, image_size::ImageSize};

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorOutput {
    Rgb,
    Hex,
}
