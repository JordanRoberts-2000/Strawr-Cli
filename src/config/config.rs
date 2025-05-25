use crate::{
    commands::img::ImgConfig,
    services::{editor_launcher::Editor, prompt::PasswordDisplay},
    template::TemplateConfig,
};
use validator::Validate;

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CliConfig {
    pub default_editor: Editor,
    pub quiet_mode: bool,
    pub password_input_display_mode: PasswordDisplay,
    // pub grab: GrabConfig,
    #[validate(nested)]
    pub img: ImgConfig,
    pub template: TemplateConfig,
}
