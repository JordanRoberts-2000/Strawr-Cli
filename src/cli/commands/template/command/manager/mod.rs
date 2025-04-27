use std::path::PathBuf;

use crate::{cli::commands::template::TemplateError, error::io::IoError, state::AppContext};

use super::execute::TemplateInput;

pub mod create;
pub mod inject;
pub mod no_input;
pub mod open;
pub mod select;

pub struct TemplateManager<'a, T: TemplateInput> {
    pub ctx: &'a AppContext,
    pub input: &'a T,
    pub templates_path: PathBuf,
}

impl<'a, T: TemplateInput> TemplateManager<'a, T> {
    pub fn new(ctx: &'a AppContext, input: &'a T) -> Result<Self, TemplateError> {
        let templates_path = ctx.storage_dir.join("templates");

        if !templates_path.exists() {
            std::fs::create_dir(&templates_path)
                .map_err(|e| IoError::CreateDir(e, templates_path.clone()))?;
            log::info!("Created templates folder at {:?}", templates_path);
        }

        Ok(Self {
            ctx,
            input,
            templates_path,
        })
    }
}
