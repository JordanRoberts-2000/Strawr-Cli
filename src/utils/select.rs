use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    Select,
};
use std::fmt::Display;

pub fn select<T: Display>(options: Vec<T>, title: &str) -> Select<T> {
    let mut render_config: RenderConfig = RenderConfig::default();

    render_config.selected_option = Some(StyleSheet::new().with_fg(Color::White));
    render_config.option = StyleSheet::new().with_fg(Color::DarkGrey);

    Select::new(title, options)
        .without_help_message()
        .with_page_size(4)
        .with_render_config(render_config)
}
