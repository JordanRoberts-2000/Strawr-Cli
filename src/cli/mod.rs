use clap::Parser;
use std::{fs, path::PathBuf};

use crate::{commands::Commands, config::constants::CONFIG_FOLDER_NAME};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(
        short,
        long,
        value_name = "FOLDERPATH",
        help = "Path to store configuration files"
    )]
    pub config_location: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}

impl Cli {
    pub fn get_config_path(&self) -> PathBuf {
        match &self.config_location {
            Some(config_path) => {
                if config_path.is_dir() {
                    return config_path.clone();
                } else if config_path.exists() {
                    panic!(
                        "Error: The provided config path '{}' is not a directory.",
                        config_path.display()
                    );
                } else {
                    fs::create_dir_all(config_path).unwrap_or_else(|err| {
                        panic!(
                            "Error: Unable to create the config directory '{}': {}",
                            config_path.display(),
                            err
                        );
                    });
                    return config_path.clone();
                }
            }
            None => {
                let default_config_path = dirs::home_dir()
                    .expect("Error: Unable to locate the home directory, to set one manually use: --config-location <FOLDERPATH>")
                    .join(CONFIG_FOLDER_NAME);

                if !default_config_path.exists() {
                    fs::create_dir_all(&default_config_path).unwrap_or_else(|err| {
                        panic!(
                            "Error: Unable to create the default config directory '{}': {}",
                            default_config_path.display(),
                            err
                        );
                    });
                }

                return default_config_path;
            }
        }
    }
}
