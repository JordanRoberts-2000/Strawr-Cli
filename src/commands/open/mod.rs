use crate::state::AppContext;

use opener;
use std::process::Command;

#[derive(clap::Parser, Debug)]
pub struct OpenCommand {
    #[arg(short, long, help = "Open the configuration folder in file explorer")]
    pub explorer: bool,

    #[arg(
        short,
        long,
        help = "Open the configuration folder in Visual Studio Code"
    )]
    pub vscode: bool,

    #[arg(short, long, help = "Print the configuration folder path")]
    pub path: bool,
}

impl OpenCommand {
    pub fn handle_command(&self, ctx: &AppContext) {
        if self.explorer {
            println!("Opening the configuration folder in file explorer...");
            if let Err(err) = opener::open(&ctx.storage_dir) {
                eprintln!("Error: Failed to open the folder in file explorer: {}", err);
            }
            return;
        }

        if self.vscode {
            println!("Opening the configuration folder in Visual Studio Code...");
            if let Err(err) = Command::new("code").arg(&ctx.storage_dir).spawn() {
                eprintln!("Error: Failed to open the folder in VSCode: {}", err);
            }
            return;
        }

        if self.path {
            println!("Configuration folder path: {}", ctx.storage_dir.display());
            return;
        }

        // Default to VSCode if no flag is provided
        println!("No flag provided. Defaulting to opening the configuration folder in Visual Studio Code...");
        if let Err(err) = Command::new("code").arg(&ctx.storage_dir).spawn() {
            eprintln!("Error: Failed to open the folder in VSCode: {}", err);
        }
    }
}
