use crate::cli::commands::grab::{GrabError, GrabManager};
use crate::error::ParseError;
use crate::state::AppContext;
use std::fs;

use super::args::GrabCommand;

impl GrabCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), GrabError> {
        let mut manager = GrabManager::new();
        manager.initialize_data_folder(ctx)?;

        let json_data = fs::read_to_string(&manager.json_file_path).map_err(|e| GrabError::Io {
            source: e,
            context: format!("failed to read {:?}", manager.json_file_path),
        })?;
        manager.data_map = serde_json::from_str(&json_data).map_err(|e| ParseError::Json {
            source: e,
            title: format!("{:?}", manager.json_file_path),
        })?;
        log::trace!("Json data loaded into memory");

        if self.list {
            return manager.open_list_file();
        }

        let key = match &self.key {
            Some(k) => Ok(k),
            None => Err(GrabError::KeyValueMissing),
        }?;

        if self.delete {
            return manager
                .delete_entry(key)
                .inspect_err(|_| log::error!("Error deleting entry"));
        }

        let encrypt = match self.encrypt {
            Some(encrypt) => encrypt,
            None => ctx.config.grab.encrypt_values_by_default,
        };

        if let Some(ref value) = self.value {
            return manager.set_entry(key, value, &encrypt);
        }

        match manager.data_map.get(key) {
            Some(val) => manager.get_entry(val),
            None => Err(GrabError::KeyNotFound {
                key: key.to_string(),
            }
            .into()),
        }?;

        Ok(())
    }
}
