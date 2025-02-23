use std::{fs, process::Command};

use args::PlaygroundCommand;

use crate::state::AppContext;

pub mod args;

impl PlaygroundCommand {
    pub fn handle_command(&self, ctx: &AppContext) {
        log::debug!("Playground Command Called");

        // Define the paths
        let playground_data_path = ctx.config_path.join("playground_data");
        let playground_path = playground_data_path.join("playground");

        // Ensure `playground_data` exists
        if !playground_data_path.exists() {
            log::debug!("playground_data folder does not exist. Creating it...");
            if let Err(err) = fs::create_dir_all(&playground_data_path) {
                eprintln!(
                    "Error: Failed to create playground_data folder '{}': {}",
                    playground_data_path.display(),
                    err
                );
                return;
            }
            log::debug!("playground_data folder created.");
        } else {
            log::debug!("playground_data folder already exists.");
        }

        // Ensure `playground` exists inside `playground_data`
        if !playground_path.exists() {
            log::debug!("playground folder does not exist. Creating it...");
            if let Err(err) = fs::create_dir_all(&playground_path) {
                eprintln!(
                    "Error: Failed to create playground folder '{}': {}",
                    playground_path.display(),
                    err
                );
                return;
            }
            log::debug!("playground folder created.");
        } else {
            log::debug!("playground folder already exists.");
        }

        // Open the `playground` folder in VSCode
        log::debug!("Opening playground folder in VSCode...");
        if let Err(err) = Command::new("code").arg(&playground_path).spawn() {
            eprintln!(
                "Error: Failed to open playground folder in VSCode '{}': {}",
                playground_path.display(),
                err
            );
        } else {
            println!(
                "Opened playground folder in VSCode: {}",
                playground_path.display()
            );
        }
    }
}
