use std::path::PathBuf;

use crate::{
    cli::commands::template::TemplateError, error::io::IoError, state::AppContext, utils::Editor,
};

pub mod inject;
pub mod open;
pub mod select;

pub struct TemplateManager<'a> {
    pub ctx: &'a AppContext,
    pub templates_path: PathBuf,
    pub editor: &'a Editor,
}

impl<'a> TemplateManager<'a> {
    pub fn new(ctx: &'a AppContext, editor: &'a Editor) -> Result<Self, TemplateError> {
        let templates_path = ctx.storage_dir.join("templates");

        if !templates_path.exists() {
            std::fs::create_dir_all(&templates_path)
                .map_err(|e| IoError::CreateDir(e, templates_path.clone()))?;
            log::info!("Created templates folder at {:?}", templates_path);
        }

        Ok(Self {
            ctx,
            templates_path,
            editor,
        })
    }
}
