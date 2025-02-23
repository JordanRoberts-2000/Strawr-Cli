use std::fs;

use crate::{commands::uninstall::UninstallCommand, state::AppContext};

impl UninstallCommand {
    pub fn delete_config_folder(&self, ctx: &AppContext) {
        match fs::remove_dir_all(&ctx.config_path) {
            Ok(_) => {
                log::debug!("Successfully deleted '{}'.", ctx.config_path.display());
            }
            Err(err) => {
                eprintln!(
                    "Error: Failed to delete the config directory '{}': {}",
                    ctx.config_path.display(),
                    err
                );
            }
        }
    }
}
