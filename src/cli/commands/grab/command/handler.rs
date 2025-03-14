use crate::cli::commands::grab::GrabManager;
use crate::error::{Error, ParseError, Result};
use crate::state::AppContext;
use std::fs;

use super::args::GrabCommand;

impl GrabCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let mut manager = GrabManager::new();
        manager.initialize_data_folder(ctx)?;

        let json_data = fs::read_to_string(&manager.json_file_path)
            .map_err(|e| Error::Io(e, format!("failed to read {:?}", manager.json_file_path)))?;
        manager.data_map = serde_json::from_str(&json_data).map_err(|e| {
            Error::Parse(
                ParseError::Json(e),
                format!("failed to parse {:?}", manager.json_file_path),
            )
        })?;
        log::trace!("Json data loaded into memory");

        if self.list {
            return manager.open_list_file();
        }

        let key = match &self.key {
            Some(k) => k,
            None => {
                log::error!("Key should have a value but does't");
                return Err(Error::Internal);
            }
        };

        if self.delete {
            return manager.delete_entry(key);
        }

        let encrypt = match self.encrypt {
            Some(encrypt) => encrypt,
            None => ctx.config.grab.encrypt_values_by_default,
        };

        if let Some(ref value) = self.value {
            return manager.set_entry(key, value, &encrypt);
        }

        if let Some(val) = manager.data_map.get(key) {
            return manager.get_entry(val);
        } else {
            println!("Key '{}' not found", key);
        }

        Ok(())
    }
}
