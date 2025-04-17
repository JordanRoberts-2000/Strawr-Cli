use std::{env, fs};

use uuid::Uuid;

use crate::state::AppContext;

use super::{args::TempCommand, error::TempError};

impl TempCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TempError> {
        let editor = self.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        let mut temp_dir_path = env::temp_dir();
        temp_dir_path.push(format!("temp-session-{}", Uuid::new_v4()));

        let temp_dir_path = temp_dir_path.join("temporary");

        fs::create_dir_all(&temp_dir_path).map_err(TempError::TempDirCreation)?;

        log::info!("Created temp dir at: {}", temp_dir_path.display());

        editor.open(&temp_dir_path)?;

        Ok(())
    }
}
