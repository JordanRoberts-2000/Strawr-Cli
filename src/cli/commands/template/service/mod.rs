use std::path::Path;

use crate::{
    traits::ToService,
    utils::{editor::EditorLauncher, input::CliInput},
};

pub mod editor;
pub mod inject;
pub mod input;
pub mod template;
pub mod variants;

pub struct TemplateService<'a> {
    pub input: &'a dyn CliInput,
    pub editor_launcher: &'a dyn EditorLauncher,
    pub templates_path: &'a Path,
}

impl<'a> TemplateService<'a> {
    pub fn new(
        input: &'a dyn CliInput,
        editor_launcher: &'a dyn EditorLauncher,
        templates_path: &'a Path,
    ) -> Self {
        Self {
            input,
            editor_launcher,
            templates_path,
        }
    }

    pub fn from<T>(ctx: &'a T) -> Self
    where
        T: ToService<'a, Self> + ?Sized,
    {
        ctx.to_service()
    }
}
