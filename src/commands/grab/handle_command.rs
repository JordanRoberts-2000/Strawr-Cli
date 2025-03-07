use crate::commands::grab::service::GrabService;
use crate::error::{Error, ParseError, Result};
use crate::state::AppContext;
use std::fs;

use super::GrabCommand;

impl GrabCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<()> {
        log::trace!("Grab Command Called");

        let mut service = GrabService::new();
        service.initialize_data_folder(ctx)?;

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

        let encrypt = match self.encrypt {
            Some(encrypt) => encrypt,
            None => ctx.config.grab.encrypt_values_by_default,
        };

        if let Some(ref value) = self.value {
            return service.set_entry(key, value, &encrypt);
        }

        if let Some(val) = service.data_map.get(key) {
            return service.get_entry(val);
        }

        println!("Key '{}' not found", key);

        Ok(())
    }
}
