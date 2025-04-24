use std::path::PathBuf;

use crate::{cli::commands::template::TemplateError, state::AppContext};

pub mod create;
pub mod inject;
pub mod no_input;
pub mod open;

pub struct TemplateManager<'a> {
    pub ctx: &'a AppContext,
    pub templates_path: PathBuf,
}

impl<'a> TemplateManager<'a> {
    pub fn new(ctx: &'a AppContext) -> Result<Self, TemplateError> {
        let templates_path = ctx.storage_dir.join("templates");

        if !templates_path.exists() {
            std::fs::create_dir(&templates_path).map_err(|e| TemplateError::Io {
                source: e,
                context: format!("Failed to create templates folder at {:?}", templates_path),
            })?;
            log::info!("Created templates folder at {:?}", templates_path);
        }

        Ok(Self {
            ctx,
            templates_path,
        })
    }
}
