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

    #[arg(short, long)]
    pub encrypt: Option<bool>,

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
            return service.open_list_file();
        }

        let key = match &self.key {
            Some(k) => k,
            None => return Err(Error::Custom("Key was not found".to_string())),
        };

        if self.delete {
            return service.delete_entry(key);
        }

        if let Some(ref value) = self.value {
            return service.set_entry(key, value);
        }

        if let Some(val) = service.data_map.get(key) {
            return service.get_entry(val);
        }

        println!("Key '{}' not found", key);

        Ok(())
    }
}
