use inquire::ui::{Color, RenderConfig, StyleSheet};

use super::InputError;

pub mod standard;
pub mod without_filter;

pub fn build_render_config<'a>() -> RenderConfig<'a> {
    let mut config = RenderConfig::default();
    config.selected_option = Some(StyleSheet::new().with_fg(Color::White));
    config.option = StyleSheet::new().with_fg(Color::DarkGrey);
    config
}
