use std::path::PathBuf;

use crate::{
    cli::commands::template::{TemplateConfig, TemplateError},
    state::AppContext,
    utils::{fs::subfolders, Editor},
};

pub mod create;
pub mod inject;
pub mod no_input;
pub mod open;

pub struct TemplateManager {
    pub templates_path: PathBuf,
    pub config: TemplateConfig,
    pub editor: Editor,
    pub templates: Vec<String>,
}

impl TemplateManager {
    pub fn new(ctx: &AppContext) -> Result<Self, TemplateError> {
        let templates_path = ctx.storage_dir.join("templates");

        if !templates_path.exists() {
            std::fs::create_dir(&templates_path).map_err(|e| TemplateError::Io {
                source: e,
                context: format!("Failed to create templates folder at {:?}", templates_path),
            })?;
            log::info!("Created templates folder at {:?}", templates_path);
        }

        let templates =
            subfolders(&templates_path).map_err(TemplateError::FailedToReadTemplateDir)?;

        Ok(Self {
            templates_path,
            config: ctx.config.template.clone(),
            editor: ctx.config.default_editor.clone(),
            templates,
        })
    }
}
