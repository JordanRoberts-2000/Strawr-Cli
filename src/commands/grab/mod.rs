use arboard::Clipboard;
use service::GrabService;
use std::fs;
use validation::{validate_key, validate_value};

pub mod service;
pub mod validation;

use crate::error::{Error, ParseError, Result};
use crate::state::AppContext;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    #[arg(required_unless_present = "list", value_parser = validate_key)]
    pub key: Option<String>,

    #[arg(short, long)]
    pub delete: bool,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long, value_parser = validate_value)]
    pub value: Option<String>,
}

impl GrabCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<()> {
        log::debug!("Grab Command Called");

        let mut service = GrabService::new();
        service.initialize_data_folder(&ctx)?;

        let json_data = fs::read_to_string(&service.json_file_path)
            .map_err(|e| Error::Io(e, "failed to read grab/data.json".to_string()))?;
        service.data_map = serde_json::from_str(&json_data)
            .map_err(|e| Error::Parse(ParseError::Json(e), "failed to ".to_string()))?;

        if self.list {
            service.open_list_file()?;
        } else if self.delete {
            service.delete_entry(self.key.as_ref().unwrap())?;
        } else if let Some(ref value) = self.value {
            service.set_entry(self.key.as_ref().unwrap(), value)?;
        } else if let Some(val) = service.data_map.get(self.key.as_ref().unwrap()) {
            log::debug!("Value '{}' copied to clipboard", val);
            let mut clipboard = Clipboard::new()
                .map_err(|e| Error::Custom(format!("Failed to access clipboard, {}", e)))?;
            clipboard
                .set_text(val)
                .map_err(|e| Error::Custom(format!("Failed to save text to clipboard, {}", e)))?;
        } else {
            println!("Key '{}' not found", self.key.as_ref().unwrap());
        }

        Ok(())
    }
}
