use args::PlaygroundCommands;
use dirs;

use crate::state::AppContext;

pub mod args;

impl PlaygroundCommands {
    pub fn handle_command(&self, ctx: &AppContext) {
        println!("Hello form playground");
        if let Some(home_dir) = dirs::home_dir() {
            println!("Home directory: {}", home_dir.display());
        }
    }
}
