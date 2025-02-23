use std::fs;

use arboard::Clipboard;
use service::GrabService;

pub mod service;

use crate::state::AppContext;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    #[arg(required_unless_present = "list")]
    pub key: Option<String>,

    #[arg(short, long)]
    pub delete: bool,

    #[arg(short, long)]
    pub list: bool,

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

        if self.list {
            service.open_list_file()?;
        } else if self.delete {
            service.delete_entry(self.key.as_ref().unwrap())?;
        } else if let Some(ref value) = self.value {
            service.set_entry(self.key.as_ref().unwrap(), value)?;
        } else if let Some(val) = service.data_map.get(self.key.as_ref().unwrap()) {
            log::debug!("Value '{}' copied to clipboard", val);
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(val)?;
        } else {
            println!("Key '{}' not found", self.key.as_ref().unwrap());
        }

        Ok(())
    }
}
