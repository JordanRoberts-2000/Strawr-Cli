use args::PlaygroundCommands;
use dirs;

pub mod args;

impl PlaygroundCommands {
    pub fn handle_command(&self) {
        println!("Hello form playground");
        if let Some(home_dir) = dirs::home_dir() {
            println!("Home directory: {}", home_dir.display());
        }
    }
}
