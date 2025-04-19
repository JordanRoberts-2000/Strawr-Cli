use std::path::PathBuf;

use crate::{cli::commands::template::TemplateConfig, state::AppContext};

pub mod storage;

pub struct TemplateManager {
    pub folder_path: PathBuf,
    pub config: TemplateConfig,
    pub templates: Vec<String>,
}

impl TemplateManager {
    pub fn new(ctx: &AppContext) -> Self {
        Self {
            folder_path: ctx.storage_dir.join("templates"),
            config: ctx.config.template.clone(),
            templates: Vec::new(),
        }
    }
}
