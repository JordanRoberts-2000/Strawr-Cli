use clap::Parser;

use crate::state::AppContext;

pub mod utils;

#[derive(Parser, Debug)]
pub struct UninstallCommand;

impl UninstallCommand {
    pub fn handle_command(&self, ctx: &AppContext) {
        log::debug!("Uninstall Command called");

        if self.confirm_uninstallation(ctx) {
            log::debug!("Uninstallation confirmed");

            self.delete_config_folder(ctx);
            // todo: delete binary

            println!("Uninstallation complete")
        } else {
            println!("Uninstallation cancelled.");
        }
    }
}
