use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    InquireError, Select,
};

use crate::services::prompt::user::UserInputError;

fn render_config<'a>() -> RenderConfig<'a> {
    let mut config = RenderConfig::default();
    config.selected_option = Some(StyleSheet::new().with_fg(Color::White));
    config.option = StyleSheet::new().with_fg(Color::DarkGrey);
    config
}

pub fn select_prompt(options: &[String], msg: &str) -> Result<String, UserInputError> {
    let prompt = Select::new(msg, options.to_vec())
        .without_filtering()
        .without_help_message()
        .with_page_size(4)
        .with_render_config(render_config())
        .prompt();

    match prompt {
        Ok(selected) => Ok(selected.to_string()),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            Err(UserInputError::Canceled)
        }
        Err(err) => Err(UserInputError::InquireError(err)),
    }
}
