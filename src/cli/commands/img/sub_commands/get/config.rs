use crate::cli::commands::img::config::ColorOutput;

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgGetConfig {
    pub default_color_output: ColorOutput,
}
