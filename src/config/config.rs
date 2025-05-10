use crate::{services::editor_launcher::Editor, template::TemplateConfig};

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CliConfig {
    pub default_editor: Editor,
    // pub grab: GrabConfig,
    // #[validate(nested)]
    // pub img: ImgConfig,
    pub template: TemplateConfig,
}
