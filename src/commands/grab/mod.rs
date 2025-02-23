use std::fs;

use arboard::Clipboard;
use service::GrabService;

pub mod service;

use crate::state::AppContext;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    pub key: String,

    #[arg(short, long)]
    pub delete: bool,

    #[arg(short, long)]
    pub value: Option<String>,
}

impl GrabCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Grab Command Called");

        let mut service = GrabService::new();
        service.initialize_data_folder(&ctx)?;

        let json_data = fs::read_to_string(&service.json_file_path)?;
        service.data_map = serde_json::from_str(&json_data)?;

        if self.delete {
            service.delete_entry(&self.key)?;
        } else if let Some(ref value) = self.value {
            service.set_entry(&self.key, value)?;
        } else if let Some(val) = service.data_map.get(&self.key) {
            log::debug!("Value '{}' copied to clipboard", val);
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(val)?;
        } else {
            println!("Key '{}' not found", self.key);
        }

        Ok(())
    }
}
